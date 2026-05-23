---
title: "On the Theory of Games of Strategy"
date: "2026-05-23T17:49:26"
---

This is my attempt to transcribe this PDF into markdown by hand. I will do about one or two pages a day. 

[Source Text](https://cs.uwaterloo.ca/~y328yu/classics/vonNeumann.pdf)

# ON THE THEORY OF GAMES OF STRATEGY¹

John von Neumann

A translation by Mrs. Sonya Bargmann of
"Zur Theorie der Gesellschaftsspiele," Mathematische Annalen 100 (1928), pp. 295-320.

## INTRODUCTION

1. The present paper is concerned with the following question:
n players S₁, S₂, ..., Sₙ are playing a given game of strategy ω. How must one of the participants, Sₘ, play in order to achieve a most advantageous result?

The problem is well known, and there is hardly a situation in daily life into which this problem does not enter. Yet, the meaning of this question is not unambiguous. For, as soon as n > 1 (i.e., ω is a game of strategy in the proper sense), the fate of each player depends not only on his own actions but also on those of the others, and their behavior is motivated by the same selfish interests as the behavior of the first player. We feel that the situation is inherently circular.

Hence we must first endeavor to find a clear formulation of the question. What, exactly, is a game of strategy? A great many different things come under this heading, anything from roulette to chess, from baccarat to bridge. And after all, any event — given the external conditions and the participants in the situation (provided the latter are acting of their own free will) — may be regarded as a game of strategy if one looks at the effect it has on the participants.² What element do all these things have in common?

We may assume that it is the following:

A game of strategy consists of a certain series of events each of which may have a finite number of distinct results. In some cases, the outcome depends on chance, i.e., the probabilities with which each of the possible results will occur are known, but nobody can influence them. All other events depend on the free decision of the players S₁, S₂, ..., Sₙ. In other words, for each of these events it is known which player, Sₘ, determines its outcome and what is his state of information with respect to the results of other ("earlier") events at the time when he makes his decision. Eventually, after the outcome of all events is known, one can calculate according to a fixed rule what payments the players S₁, S₂, ..., Sₙ must make to each other. It is easy to bring this somewhat qualitative explanation into a precise form. The definition of a game of strategy would then be the following:

For a complete description of a game ω, the following data are necessary, which in their entirety are the "rules of the game."

α) The number of events or "draws" depending on chance, and the number of events or "steps" depending on the free decision of the individual players must be specified. Let these numbers be z and s respectively, and let us denote the "draws" by E₁, E₂, ..., E_z, the "steps" by F₁, F₂, ..., F_s.

β) The number of possible results of each "draw", E_μ, and of each "step", F_ν, must be specified. Let these numbers be M_μ and N_ν respectively. (μ = 1, 2, ..., z, ν = 1, 2, ..., s.) We shall denote the results, for short, by their numbers 1, 2, ..., M_μ and 1, 2, ..., N_ν respectively.

γ) For each "draw" E_μ the probabilities α_μ^(1), α_μ^(2), ..., α_μ^(M_μ) of the different results 1, 2, ..., M_μ must be given. Obviously, we have

α_μ^(1) ≥ 0, α_μ^(2) ≥ 0, ..., α_μ^(M_μ) ≥ 0 

α_μ^(1) + α_μ^(2) + ... + α_μ^(M_μ) = 1
