#import "lib.typ": *
#import "logo.typ": logo

#set page(
  width: 1280pt,
  height: 640pt,
  margin: 100pt,
  fill: background,
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
