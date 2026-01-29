#import "lib.typ": *
#import "logo.typ": logo

#set page(
  width: 1073pt,
  height: 151pt,
  fill: none,
  background: box(
    width: 100%,
    height: 100%,
    fill: background,
    radius: 10%,
  ),
)

#set text(
  size: 90pt,
  font: font,
  fill: foreground,
)

#set align(center + horizon)

#stack(
  dir: ltr,
  spacing: 1em,
  logo(height: 100pt),
  [skrytsam],
  logo(height: 100pt),
)
