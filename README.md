<h1 align="center">
Tags
</h1>

<br />

Tags are metadata for Golang struct fields. In this project we are going to create a new
tag for struct fields, in order to make them appear or not in encoding/json marshal.

## Install

Get the module by ```go get github.com/amirhnajafiz/tags@latest```.

## Values

- none
- display

### Example

```go

type Sample struct {
  Id       int    `json:"id"`
  Value    string `json:"value"`
  Metadata string `json:"metadata" tag:"none"`
}

```
