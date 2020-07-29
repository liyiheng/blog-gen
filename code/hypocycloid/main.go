package main

import (
	"bytes"
	"image"
	"image/color"
	"image/png"
	"math"
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
)

func main() {
	app := gin.Default()
	app.GET("/draw", func(ctx *gin.Context) {
		outerRStr := ctx.DefaultQuery("outer_r", "100")
		innerRStr := ctx.DefaultQuery("inner_r", "8")
		distStr := ctx.DefaultQuery("dist", "60")
		squareStr := ctx.DefaultQuery("square_size", "360")
		redStr := ctx.DefaultQuery("r", "0")
		greenStr := ctx.DefaultQuery("g", "200")
		blueStr := ctx.DefaultQuery("b", "0")
		outerR, e1 := strconv.Atoi(outerRStr)
		innerR, e2 := strconv.Atoi(innerRStr)
		dist, e3 := strconv.Atoi(distStr)
		square, e4 := strconv.Atoi(squareStr)
		red, e5 := strconv.Atoi(redStr)
		green, e6 := strconv.Atoi(greenStr)
		blue, e7 := strconv.Atoi(blueStr)
		if e1 != nil || e2 != nil || e3 != nil || e4 != nil || e5 != nil || e6 != nil || e7 != nil {
			ctx.AbortWithStatus(http.StatusBadRequest)
			return
		}

		p := &pannel{}
		p.outerR = float64(outerR)
		p.innerR = float64(innerR)
		p.distance = float64(dist)
		p.img = image.NewRGBA(image.Rect(0, 0, square, square))
		step := 3.14 / 720
		r := uint8(red)
		g := uint8(green)
		b := uint8(blue)
		for i := 0; i < 72000; i++ {
			p.step(step, r, g, b)
		}
		buf := bytes.NewBuffer(make([]byte, 0))
		_ = png.Encode(buf, p.img)
		ctx.ContentType()
		ctx.Header("Content-Type", "image/png")
		ctx.Writer.Write(buf.Bytes())
		ctx.Writer.Flush()
	})
	app.StaticFile("/", "./index.html")
	app.Run(":8080")
}

type pannel struct {
	outerR   float64
	innerR   float64
	distance float64
	alpha    float64
	beta     float64
	img      *image.RGBA
}

func (p *pannel) step(delta float64, r, g, b uint8) {
	p.alpha += delta

	a := delta * p.outerR
	beta := a / p.innerR
	p.beta += beta

	xxx := p.beta - p.alpha
	deltaY := math.Sin(xxx) * p.distance
	deltaX := math.Cos(xxx) * p.distance

	centerY := math.Sin(p.alpha) * (p.outerR - p.innerR)
	centerX := math.Cos(p.alpha) * (p.outerR - p.innerR)

	x := int(centerX-deltaX) + p.img.Bounds().Max.X/2
	y := int(centerY-deltaY) + p.img.Bounds().Max.Y/2
	p.img.SetRGBA(x, y, color.RGBA{r, g, b, 255})
}
