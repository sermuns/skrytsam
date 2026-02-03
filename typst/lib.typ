#let card(body) = {
  set page(
    width: auto,
    height: auto,
    margin: 1em,
    fill: blue.desaturate(50%).darken(75%),
  )
  set text(
    font: "Monaspace Krypton",
    fill: luma(90%),
  )
  set grid(
    gutter: 1em,
  )
  show heading: set block(below: 1em)

  body
}
