# Lens development

A **Lens** is a pluggable module in Spyglass that provides searchable entries, actions, and behavior when interacting with results. Lenses are the "tabs" in the main Spyglass UI.

## Overview

A Lens must implement the `lens.Lens` interface:

```go
type Lens interface {
	Name() string
	Search(query string) ([]Entry, error)
	Enter(entry Entry) error
	ContextActions(entry Entry) []Action
}
```

## Core Types

### Entry

Represents a **single item** shown in the results list

```go
type Entry struct {
	ID          string
	Title       string
	Icon        string
	Description string
}
```

- *ID*: Unique identifier (often a path, URL, or internal ID)
- *Title*: Main display text
- *Icon*: A single character (e.g. Nerd Font icon)
- *Description*: Shown in the bottom panel when selected

### Action

Represents a **context menu** (accessed with Shift+Tab) action for an entry

```go
type Action struct {
	Name string
	Run  func(Entry) error
}
```

- *Name*: Displayed in the context menu
- *Run*: Function executed when the action is selected

## Required methods

### `Name() string`

Returns the name of the Lens, used in the **tab bar** at the top of the Spyglass interface.

```go
func (l *myLens) Name() string {
	return "My Lens"
}
```

### `Search(query string) ([]Entry, error)`

Called whenever the user types into the search bar

- Should return a filtered list of entries
- Called frequently (every time a character in the search bar changes), so it should be *fast*
- If the query is empty, default/unfiltered results should be returned

```go
func (l *myLens) Search(query string) ([]lens.Entry, error) {
	var results []lens.Entry

	for _, item := range l.items {
		if query == "" || strings.Contains(item.Name, query) {
			results = append(results, lens.Entry{
				ID:    item.ID,
				Title: item.Name,
				Icon:  "",
			})
		}
	}

	return results, nil
}
```

### `Enter(entry Entry) error`

Called when the user presses **Enter** on the selected Entry

Typical uses:

- Launch an application
- Open a file
- Open a URL

```go
func (l *myLens) Enter(e lens.Entry) error {
  cmd := exec.Command("xdg-open", e.ID)
  cmd.SysProcAttr = &syscall.SysProcAttr {Setsid: true}

  return cmd.Start()
}
```

> [!WARNING]
> Spyglass exits after running the Enter command, so use `Start()` instead of `Run()` if you don't want to block.
> Also, use something like `& sleep 5` at the end of the command to prevent the terminal from quitting before the application detaches from the terminal

### `ContextActions(entry Entry) []Action`

Returns **context menu actions** (opened with Shift+Tab)

```go
func (l *myLens) ContextActions(e lens.Entry) []lens.Action {
	return []lens.Action{
		{
			Name: "Copy ID",
			Run: func(entry lens.Entry) error {
				return copyToClipboard(entry.ID)
			},
		},
	}
}
```

If no actions are needed, then `nil` or an empty slice is returned.

## Creating a Lens

### 1. Create a new package

```shell
go mod init
```

### 2. Implement the Lens

```go
package myLens

import "github.com/indium114/spyglass/lens"

type myLens struct{}

func New() lens.Lens {
  return &myLens{}
}
```

Implement all of the required methods

### 3. Register the Lens

#### Clone the Spyglass repo

```shell
git clone https://github.com/indium114/spyglass
```

#### Add the Lens to `lenses.go`

```go
package main

import (
  // ...
  "github.com/youruser/myLens"
)

var Lenses = []lens.Lens{
  // ...
  myLens.New()
}
```

Lenses are registered top-to-bottom.
> If you want your Lens first in the tab bar, put it at the top of Lenses
