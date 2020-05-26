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
		if 0 == i%10000000 {
			fmt.Printf("\r%d:%v", i, time.Now().UnixNano())
		}
	}
	end := time.Now().UnixNano()
	fmt.Printf("\nDone: %v ms\n", (end-start)/1000000)
}
