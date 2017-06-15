package main

import "vidalia/runner"

func main() {
    conn, ch := runner.ObtainChannelConnection()
    runner.RunService(ch)
    ch.Close()
    conn.Close()
}
