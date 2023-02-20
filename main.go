package main

/*
#cgo LDFLAGS: ${SRCDIR}/dataloader/target/debug/libdataloader.a
#include "./dataloader/src/go.h"
*/
import "C"
import (
	"os"
	"path"
)

func main() {
	samples := 44
	buf := make([]float32, 7*44)
	C.render_go(
		C.CString(path.Join(os.Getenv("DATA_PATH"), "2022-11-14")),
		(*C.float)(&buf[0]),
		C.size_t(samples),
	)
}
