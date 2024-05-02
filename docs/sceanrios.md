# Usage scenarios

Three usage scenarios are described to illustrate how the nextsv tool might be used as part of a development process as a program progress from the initial unstable releases through to the first stable release. The second and third scenarios cover deployment of pre-release versions during the initial unstable development and as part of the release of the final first stable version.

## Scenario 1: Initial unstable to first stable version

This scenarios works through a number of unstable releases before making the initial stable release.

The first new release version is calculated based on an initial unstable version `0.1.0`.

### Add a new `feature` to version `0.1.0`

Nextsv can be used in conjunction with cargo release to calculate the release LEVEL or VERSION to release.

```ignore
cargo release $(nextsv calculate)

```

The -n option to nextsv prints the expected version number based on the calculated bump level. The -n options will be used throughout these scenarios.

```console
$ nextsv -n calculate
patch
0.1.1

```

### Make `fix` to version `0.1.1`

```console
$ nextsv -n calculate
patch
0.1.2

```

### Add a new `feature` to version `0.1.2`

```console
$ nextsv -n calculate
patch
0.1.3

```

### Add a `breaking` change to version `0.1.3`

As our current version is unstable the breaking change will increment the minor component.

```console
$ nextsv -n calculate
minor
0.2.0

```

### Promote to `first` stable version from version `0.2.0`

As there is no bump level in cargo release, nextsv produces the version number to effect the version 1.0.0 release.

```console
nextsv -n force first
1.0.0
1.0.0

```

## Scenario 2: Unstable pre-release versions

In this scenario pre-releases are created as part of the development of unstable software build.

### Add a `feature` to version `0.3.0` and make an `alpha` pre-release version

When a pre-release is made the version number change is also calculated and applied.

The feature level change promotes the version to 0.4.0 and the alpha tag is then appended to that version.

```console
$ nextsv -n force alpha
alpha
0.3.1-alpha.1
```

### Add a `fix` to release `0.3.1-alpha.1`

A fix to a pre-release version will increment the pre-release version number only.

```console
$ nextsv -n calculate 
alpha
0.3.1-alpha.2

```

### Add another `fix` to version `0.3.1-alpha.2` and release as the first `beta` release

As the change is made to a pre-release version the version number is not affected.

```console
$ nextsv -n force beta
beta
0.3.1-beta.1

```

### Add another `fix` to version `0.3.1-beta.1` and release as the first `rc` release

As the change is made to a pre-release version the version number is not affected.

```console
$ nextsv -n force rc
beta
0.3.1-rc.1

```

### Add final `fix` to release `0.3.1-rc.1` and release as `first` stable version

Forcing the version to first will remove the pre-release tag from the version number and give the release the first stable version number.

```console
$ nextsv -n force first
1.0.0
1.0.0

```

## Scenario 3: Initial release to production with pre-releases

This scenario starts the final rc release of version 0.4.0 and releases that version. This is followed by a series of pre-release versions leading to the first stable version.

### Add a `fix` to version `0.4.0-rc.1` and `release` it

Releasing a version takes the pre-release tag off the version number without changing the version number.

```console
$ nextsv -n force release
release
0.4.0

```

### Add `feature` to version `0.4.0` and release `rc` for `first` stable version

Update version 0.4.0 and release as an rc pre-release for the first stable version.

The bump is output as a version number for cargo release as there is no level for case.

```console
$ nextsv -n force rc --first
1.0.0-rc.1
1.0.0-rc.1

```

### Add `fix` to version `1.0.0-rc.1`

A fix is a simple calculation and bump of rc level results.

```console
$ nextsv -n calculate
rc
1.0.0-rc.2

```

### Apply final `fix` to version `1.0.0-rc.2` and `release` as the first stable version

```console
$ nextsv -n force release 
release
1.0.0

```
