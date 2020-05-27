package main

import (
	"fmt"
	sha256 "github.com/minio/sha256-simd"
	"time"
)

func main() {
	buf := make([]byte, 9728)
	start := time.Now().UnixNano()
	for i := 0; i < 65535; i++ {
		buf[0] = byte(i)
		sha256.Sum256(buf)
		if 0 == i%1000000 {
			fmt.Printf("\r%d:%v/%v ms", i, time.Now().UnixNano(),
				(time.Now().UnixNano() - start)/1000000)
		}
	}
	end := time.Now().UnixNano()
	fmt.Printf("\nDone: %v ms\n", (end-start)/1000000)
}
