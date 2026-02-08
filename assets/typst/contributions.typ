#import "lib.typ": card, default-color
#let color = sys.inputs.at("color", default: default-color)
#show: card.with(color: color)

#import "@preview/octique:0.1.1": octique
#let octique = octique.with(color: color)


#let today = datetime.today().display("[month repr:short] [day] [year]")

#let contributions = sys.inputs.at(
  "contributions",
  default: (
    total-num-stars-earned: 38,
    total-num-commits: 948,
    total-num-prs: 105,
    total-num-issues: 39,
  ),
)

= Contributions

#grid(
  columns: 3,
  align: horizon,

  octique("star"),
  [Total stars earned:],
  str(contributions.total-num-stars-earned),

  octique("git-commit"), [Total commits:], str(contributions.total-num-commits),

  octique("git-pull-request"), [Total PRs:], str(contributions.total-num-prs),

  octique("issue-opened"), [Total issues:], str(contributions.total-num-issues),
)
