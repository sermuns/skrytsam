#import "@preview/cetz:0.4.2": canvas
#import "@preview/cetz-plot:0.1.3": chart
#import "lib.typ": card

#show: card

#let languages = sys.inputs.at(
  "languages",
  // TODO: remove defaults!
  default: (
    Rust: (color: "#dea584", bytes: 121264),
    Typst: (color: "#239dad", bytes: 463173),
    HTML: (color: "#e34c26", bytes: 321322),
    Others: (color: "#444", bytes: 321322),
  ),
)
#let total_language_bytes = (
  languages.values().fold(0, (acc, v) => acc + v.bytes)
)
#let languages-pairs = languages.pairs()

= Most used languages

#grid(
  columns: 2,
  grid(
    columns: 3,
    align: (x, y) => horizon + if x == 2 { right } else { left },
    ..languages-pairs
      .map(((lang-name, v)) => (
        block(
          height: .8em,
          width: .8em,
          fill: rgb(v.color),
        ),
        lang-name,
        [#(calc.round(100 * v.bytes / total_language_bytes, digits: 1)) %],
      ))
      .flatten(),
  ),

  canvas({
    let colors = languages-pairs.map(((k, v)) => rgb(v.color))

    chart.piechart(
      languages-pairs.map(((k, v)) => (k, v.bytes)),
      label-key: none,
      value-key: 1,
      radius: 2,
      stroke: none,
      slice-style: colors,
    )
  }),
)
