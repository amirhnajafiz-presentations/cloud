package main

import (
	"github.com/gofiber/fiber/v2"
)

type Sample struct {
	Id       int    `json:"id" tag:"none"`
	Value    string `json:"value" tag:"display"`
	Metadata string `json:"-"`
}

func main() {
	//u := Sample{
	//	Id:    44,
	//	Value: "Value",
	//}
	//
	//t := reflect.TypeOf(u)
	//
	//for i := 0; i < t.NumField(); i++ {
	//	f := t.Field(i)
	//	if value, ok := f.Tag.Lookup("tag"); ok {
	//		fmt.Println(value)
	//	}
	//}
	app := fiber.New()

	app.Get("/", func(ctx *fiber.Ctx) error {
		s := Sample{
			Id:       44,
			Value:    "value",
			Metadata: "meta404",
		}

		return ctx.JSON(s)
	})

	if err := app.Listen(":8080"); err != nil {
		panic(err)
	}
}
