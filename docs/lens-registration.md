# Register a Lens

## Prerequisites

- `git`
- `go`
- A `text editor` (Neovim, Helix, VS Code, etc.)

## 1. Clone the repo

Adding new lenses is done in the source code, so you'll need to grab a copy of that:

```shell
git clone https://github.com/indium114/spyglass
```

Then `cd` into the repo

```shell
cd spyglass
```

## 2. Open the lens registry

Lens registration happens in the `lenses.go` file in the root of the repo.
Open it with your favourite text editor:

```shell
# Helix
hx lenses.go

# Neovim
nvim lenses.go

# VS Code
code lenses.go
```

## 3. Import the lens's package

Look up the lens you want to import, and obtain its GitHub (or other git forge) URL.
It may look something like `github.com/indium114/spyglass/lenses/applications`

Note that the first two things after `github.com` is the username and the repository. Some lenses may have extra things like a package within that repo.

First, `go get` the package:

```shell
go get <URL>
# go get github.com/indium114/spyglass/lenses/applications
```

Then, add it to the `import` list at the top of `lenses.go`:

```go
import (
  // ...
  "github.com/indium114/spyglass/lenses/applications" // Insert your URL here
)
```

## 4. Add the lens to `Lenses`

Now you're ready to add the lens to the list of lenses. To do so, simply add it like so:

```go
var Lenses = []lens.Lens{
  // ...
  applications.New(), // Replace with whatever lens you're adding. Remember the comma, even if it's at the end of the list!
}
```
