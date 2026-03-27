package main

import (
	"github.com/indium114/spyglass/lens"
	"github.com/indium114/spyglass/lenses/applications"

	"github.com/indium114/spyglass/lenses/nerdfont"
	"github.com/indium114/spyglass/lenses/power"
	"github.com/indium114/spyglass/lenses/searxng"

	"github.com/indium114/spyglass/lenses/files"
)

var Lenses = []lens.Lens{
	applications.New(),
	power.New(),
	searxng.New(),
	nerdfont.New(),
	files.New(),
}
