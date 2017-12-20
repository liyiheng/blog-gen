package main

import (
	"image"
	"image/color"
	"image/draw"
	"image/png"
	"io/ioutil"
	"log"
	"math/rand"
	"os"

	"github.com/golang/freetype"
	"github.com/golang/freetype/truetype"
	"golang.org/x/image/font"
	"golang.org/x/image/math/fixed"
)

const defaultDpi = 72

var quality = 8

type Text struct {
	Content string
	Size    float64
	Color   color.RGBA
}

func parseFont(ttfPath string) (*truetype.Font, error) {
	f, e := os.Open(ttfPath)
	if e != nil {
		return nil, e
	}
	defer f.Close()
	dat, e := ioutil.ReadAll(f)
	if e != nil {
		return nil, e
	}
	fnt, e := freetype.ParseFont(dat)
	if e != nil {
		return nil, e
	}
	return fnt, nil
}

func mkCtx(dst *image.RGBA, fnt *truetype.Font) *freetype.Context {
	ctx := freetype.NewContext()
	ctx.SetSrc(image.Black)
	ctx.SetDst(dst)
	ctx.SetDPI(defaultDpi)
	ctx.SetClip(dst.Bounds())
	ctx.SetFont(fnt)
	return ctx
}

func main() {
	w := 400
	h := 200
	fontPath := "ipaexm.ttf"
	outFile := "out.png"
	bg := image.White

	rgba := image.NewRGBA(image.Rect(0, 0, w, h))
	draw.Draw(rgba, rgba.Bounds(), bg, image.ZP, draw.Src)
	fnt, e := parseFont(fontPath)
	if e != nil {
		log.Fatal(e.Error())
	}
	ctx := mkCtx(rgba, fnt)
	dat := createFakeDat()
	bgColor := colorSum(bg.C)
	for i, t := range dat {
		size := int(t.Size)
		ctx.SetFontSize(t.Size)
		ctx.SetSrc(image.NewUniform(t.Color))
		txtSize := measure(defaultDpi, t.Size, t.Content, fnt)
		topX, topY := queryIntegralImage(rgba, txtSize.Round(), size, bgColor)
		if topX < 0 || topY < 0 {
			log.Printf("no room left, %d of %d worlds finished", i, len(dat))
			break
		}
		// baseline start point of the text
		p := freetype.Pt(topX, topY+int(t.Size*3/4))
		_, e := ctx.DrawString(t.Content, p)
		if e != nil {
			log.Println(e.Error())
		}

	}

	out, e := os.Create(outFile)
	if e != nil {
		log.Println(e.Error())
		return
	}
	defer out.Close()
	e = png.Encode(out, rgba)
	if e != nil {
		log.Println(e.Error())
	}
}

func measure(dpi, size float64, txt string, fnt *truetype.Font) fixed.Int26_6 {
	opt := &truetype.Options{
		DPI:  dpi,
		Size: size,
	}
	face := truetype.NewFace(fnt, opt)

	return font.MeasureString(face, txt)
}

func colorSum(p color.Color) uint32 {
	r, g, b, a := p.RGBA()
	return r + g + b + a
}

func queryIntegralImage(img image.Image, sizeX, sizeY int, bgColor uint32) (lTopX, lTopY int) {
	if quality < 1 {
		quality = 10
	}
	size := img.Bounds().Size()
	hit := int64(0)

	foldX := size.X - sizeX
	foldY := size.Y - sizeY
	// count how many possible locations
	for i := 0; i < foldX; i++ {
		for j := 0; j < foldY; j++ {

			// Rectangle:
			//
			// 		i,j			i+sizeX,j
			//
			// 		i,j+sizeY	i+sizeX,j+sizeY
			//
			blank := true
			for x := i + sizeX; x >= i; x -= quality {
				for y := j + sizeY; y >= j; y -= quality {
					if colorSum(img.At(x, y)) != bgColor {
						blank = false
						break
					}
				}
				if !blank {
					break
				}
			}
			if !blank {
				continue
			}
			hit++

		}
	}
	if hit == 0 {
		// no room left
		return -1, -1
	}
	// pick a location at random
	goal := rand.Int63n(int64(hit))
	hit = 0
	for i := 0; i < foldX; i++ {
		for j := 0; j < foldY; j++ {
			blank := true
			for x := i + sizeX; x >= i; x -= quality {
				for y := j + sizeY; y >= j; y -= quality {
					if colorSum(img.At(x, y)) != bgColor {
						blank = false
						break
					}
				}
				if !blank {
					break
				}
			}
			if !blank {
				continue
			}
			hit++
			if hit == goal {
				return i, j
			}
		}
	}
	return -1, -1
}

func createFakeDat() []*Text {
	text := []string{
		"hello", "liyiheng", "zsh", "gnome",
		"linux", "git", "word cloud",
		"golang", "rust", "中文测试",
		"font", "baseline", "ascend", "descend", "bottom",
		"top", "leading",
	}
	ret := make([]*Text, len(text))
	for i, s := range text {
		//c := rand.Uint32()
		ret[i] = &Text{
			Size:    float64(rand.Int31n(50) + 10),
			Content: s,
			//Color:   color.RGBA{R: uint8(c), G: uint8(c >> 8), B: uint8(c >> 16)},
			Color: color.RGBA{A: 255},
		}
	}
	return ret
}
