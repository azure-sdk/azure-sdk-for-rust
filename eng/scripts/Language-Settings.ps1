$Language = "rust"
$LanguageDisplayName = "Rust"
$PackageRepository = "crates.io"
$packagePattern = "cargo-metadata.json"
#$MetadataUri = "https://raw.githubusercontent.com/Azure/azure-sdk/main/_data/releases/latest/rust-packages.csv"
$GithubUri = "https://github.com/Azure/azure-sdk-for-rust"
$PackageRepositoryUri = "https://crates.io/crates"

function SetPackageVersion ($PackageName, $Version, $ServiceDirectory, $ReleaseDate, $ReplaceLatestEntryTitle = $true) {
  if ($null -eq $ReleaseDate) {
    $ReleaseDate = Get-Date -Format "yyyy-MM-dd"
  }
  & "$EngDir/scripts/Update-PackageVersion.ps1" -ServiceDirectory $ServiceDirectory -PackageName $PackageName `
    -NewVersionString $Version -ReleaseDate $ReleaseDate -ReplaceLatestEntryTitle $ReplaceLatestEntryTitle
}

function GetExistingPackageVersions ($PackageName, $GroupId = $null) {
  try {
    $PackageName = $PackageName.ToLower()
    $response = Invoke-RestMethod -Method GET -Uri "https://crates.io/api/v1/crates/${PackageName}/versions"
    $existingVersions = $response.versions `
    | Sort-Object { [AzureEngSemanticVersion]::new($_.num) } `
    | Select-Object -ExpandProperty num
    return $existingVersions
  }
  catch {
    if ($_.Exception.Response.StatusCode -ne 404) {
      LogError "Failed to retrieve package versions for ${PackageName}. $($_.Exception.Message)"
    }
    return $null
  }
}

function Get-AllPackageInfoFromRepo ([string] $ServiceDirectory) {
  $allPackageProps = @()
  Push-Location $RepoRoot
  try {
    $searchPath = Join-Path $RepoRoot 'sdk' -Resolve

    if ($ServiceDirectory -and $ServiceDirectory -ne 'auto') {
      $searchPath = Join-Path $searchPath $ServiceDirectory -Resolve
    }

    $packages = cargo metadata --format-version 1
    | ConvertFrom-Json -AsHashtable
    | Select-Object -ExpandProperty packages
    | Where-Object { $_.manifest_path.StartsWith($searchPath) }

    $packageManifests = @{}
    foreach ($package in $packages) {
      if ($package.manifest_path -replace '\\', '/' -match '/sdk/([^/]+)/') {
        $package.ServiceDirectoryName = $Matches[1]
      }
      else {
        # ignore manifests that are not in a service directory
        continue
      }

      $package.DirectoryPath = Split-Path $package.manifest_path -Parent
      $package.DependentPackages = @()
      $packageManifests[$package.name] = $package
    }
  }
  catch {
    # This is soft error and failure is expected for python metapackages
    LogError "Failed to get all package properties"
  }
  finally {
    Pop-Location
  }

  # Invert the manifest dependency graph
  foreach ($package in $packageManifests.Values) {
    foreach ($dependency in $package.dependencies) {
      $dependencyManifest = $packageManifests[$dependency.name]
      if ($dependencyManifest) {
        $dependencyManifest.DependentPackages += $package
      }
    }
  }

  # Flatten the dependency graph recursively
  function GetDependentPackages($package, $dependantPackages) {
    if (!$dependantPackages) {
      $dependantPackages = @()
    }

    foreach ($dependency in $package.DependentPackages) {
      if ($dependantPackages.Contains($dependency)) {
        continue
      }
      $dependantPackages += $dependency
      [array]$dependantPackages = GetDependentPackages $dependency $dependantPackages
    }

    return $dependantPackages;
  }

  foreach ($package in $packageManifests.Values) {
    $pkgProp = [PackageProps]::new($package.name, $package.version, $package.DirectoryPath, $package.ServiceDirectoryName)
    $pkgProp.IsNewSdk = $true
    $pkgProp.ArtifactName = $package.name

    if ($package.name -match "mgmt") {
      $pkgProp.SdkType = "mgmt"
    }
    else {
      $pkgProp.SdkType = "client"
    }

    $pkgProp.AdditionalValidationPackages = GetDependentPackages $package | Select-Object -ExpandProperty DirectoryPath

    $allPackageProps += $pkgProp
  }

  return $allPackageProps
}

function Get-rust-PackageInfoFromPackageFile([IO.FileInfo]$pkg, [string]$workingDirectory) {
  #$pkg will be a FileInfo object for the cargo-metadata.json file in a package artifact directory
  $package = Get-Content -Path $pkg.FullName -Raw | ConvertFrom-Json
  $packageName = $package.name
  $packageVersion = $package.vers
  $docsReadMeName = $packageName -replace "^azure_" , ""

  $crateFile = Get-ChildItem $pkg.DirectoryName -Filter '*.crate'
  
  New-Item -Path $workingDirectory -ItemType Directory -Force | Out-Null
  $workFolder = Join-Path $workingDirectory $crateFile.BaseName
  if (Test-Path $workFolder) {
    Remove-item $workFolder -Recurse -Force
  }

  # This will extract the contents of the crate file into a folder matching the file name
  tar -xvzf $crateFile.FullName -C $workingDirectory

  $changeLogLoc = Get-ChildItem -Path $workFolder -Filter "CHANGELOG.md" | Select-Object -First 1
  if ($changeLogLoc) {
    $releaseNotes = Get-ChangeLogEntryAsString -ChangeLogLocation $changeLogLoc -VersionString $packageVersion
  }

  $readmeContentLoc = Get-ChildItem -Path $workFolder -Filter "README.md" | Select-Object -First 1
  if ($readmeContentLoc) {
    $readmeContent = Get-Content -Raw $readmeContentLoc
  }

  $existingVersions = GetExistingPackageVersions -PackageName $packageName

  return New-Object PSObject -Property @{
    PackageId      = $packageName
    PackageVersion = $packageVersion
    ReleaseTag     = "$packageName`_$packageVersion"
    Deployable     = $existingVersions -notcontains $packageVersion
    ReleaseNotes   = $releaseNotes
    ReadmeContent  = $readmeContent
    DocsReadMeName = $docsReadMeName
  }
}