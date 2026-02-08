#let default-color = blue.desaturate(50%)

#let card(body, color: none) = {
  let color = if color == none {
    default-color
  } else {
    color
  }

  set page(
    width: auto,
    height: auto,
    margin: 1em,
    fill: color.darken(75%),
  )
  set text(
    font: "Monaspace Krypton",
    fill: color.lighten(70%),
  )
  set grid(gutter: 1em)
  show heading: set block(below: 1em)

  body
}
