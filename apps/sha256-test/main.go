package main

import (
	"fmt"
	sha256 "github.com/minio/sha256-simd"
	"time"
)

func main() {
	buf := make([]byte, 9728)
	for i := 0; i < 9728; i++ {
		buf[i] = '1'
	}
	start := time.Now().UnixNano()
	hash := [32]byte{0}
	for i := 0; i < 1; i++ {
		// buf[0] = byte(i)
		hash = sha256.Sum256(buf)
		if 0 == i%1000000 {
			fmt.Printf("\r%d:%v/%v ms", i, time.Now().UnixNano(),
				(time.Now().UnixNano()-start)/1000000)
		}
	}
	end := time.Now().UnixNano()
	fmt.Printf("HASH input %s\n", buf)
	fmt.Printf("HASH result %x\n", hash)
	fmt.Printf("\nDone: %v ns[%v, %v]\n", end-start, start, end)
}
