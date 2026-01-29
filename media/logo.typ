#import "lib.typ": *

#set page(
  height: 500pt,
  width: 500pt,
  margin: 50pt,
  fill: none,
  background: box(
    width: 100%,
    height: 100%,
    fill: background,
    radius: 10%,
  ),
)

#let logo(height: 1.5em) = {
  set text(
    size: 200pt,
    font: font,
    fill: foreground,
  )

  set align(center + horizon)

  set image(height: height)

  place(
    center + horizon,
    dx: 0.03em,
    dy: 0.02em,
    image(
      bytes(
        read("megaphone-svgrepo-com.svg").replace(
          "#000000",
          foreground.to-hex(),
        ),
      ),
    ),
  )

  image(
    bytes(
      read("megaphone-svgrepo-com.svg").replace(
        "#000000",
        accent.to-hex(),
      ),
    ),
  )
}

#logo
