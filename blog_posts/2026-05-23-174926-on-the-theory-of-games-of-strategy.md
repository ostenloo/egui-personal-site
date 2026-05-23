---
title: "On the Theory of Games of Strategy"
date: "2026-05-23T17:49:26:z"
---

[Source Text](https://cs.uwaterloo.ca/~y328yu/classics/vonNeumann.pdf)

ON THE THEORY OF GAMES OF STRATEGY
John von Neumann
[A translation by Mrs. Sonya Bargmann of
"Zur Theorie der Gesellschaftsspiele,
Mathematische Annalen 100 (1928), pp.
295-320.]
INTRODUCTION
1. The present paper is concerned with the following question:
n players S, S, ..., Sn are playing a given game
strategy, . How must one of the participants, Sm
play in order to achieve a most advantageous result?
of
The problem is well known, and there is hardly a situation in
daily life into which this problem does not enter. Yet, the meaning of
this question is not unambiguous. For, as soon as n> 1 (i.e., is a
game of strategy in the proper sense), the fate of each player depends not
only on his own actions but also on those of the others, and their behavior is motivated by the same selfish interests as the behavior of the
first player. We feel that the situation is inherently circular.
Hence we must first endeavor to find a clear formulation of the
question. What, exactly, is a game of strategy? A great many different
things come under this heading, anything from roulette to chess, from
baccarat to bridge. And after all, any event - given the external conditions and the participants in the situation (provided the latter are
acting of their own free will) - may be regarded as a game of strategy if
one looks at the effect it has on the participants.2 What element do all
these things have in common?
A shortened version of this paper has been presented to the Goettingen
Mathematical Society on December 7, 1926.
2 This is the
the principal problem of classical economics: how is the absolutely selfish "homo economicus" going to act under given external circumstances?

We may assume that it is the following:
A game of strategy consists of a certain series of events
each of which may have a finite number of distinct results.
In some cases, the outcome depends on chance, i.e., the
probabilities with which each of the possible results will
occur are known, but nobody can influence them. All other
events depend on the free decision of the players S₁,
S,..., S. In other words, for each of these events
it is known which player, S, determines its outcome
and what is his state of information with respect to the
results of other ("earlier") events at the time when he
makes his decision. Eventually, after the outcome of
all events is known, one can calculate according to a
fixed rule what payments the players S, S,...,
must make to each other.
It is easy to bring this somewhat qualitative explanation into a
precise form. The definition of a game of strategy would then be the
following: For a complete description of a game , the following
data are necessary, which in their entirety are the "rules
of the game."
a) The number of events or "draws" depending on chance,
and the number of events or "steps" depending on the free
decision of the individual players must be specified. Let
these numbers be z and s respectively, and let us denote the "draws" by E, E₂,.., Ez, the "steps" by
F, F,.., F
B) The number of possible results of each "draw", E,
and of each "step", F, must be specified. Let these
numbers be M and N respectively. (μ = 1, 2, ..., Z,
v = 1, 2, ..., S.) We shall denote the results, for short,
by their numbers 1, 2, ..., M and 1, 2, ..., N,
respectively.
(1) 7) For each "draw" E the probabilities a,
(2) (M)
of the different results 1, 2, ..., M
must be given. Obviously, we have
(M (1) 20, a(2) 20,………、 20
(M)

6) For every "step" Fy, the player S who determines the outcome of this "step" ("whose step" F is)
must be specified. In addition, the numbers of all
"draws and steps" must be specified of whose outcome the
player is informed at the time he makes his decision
concerning F. (These "draws" and "steps" we shall
call "earlier" than F.)
In order that this whole scheme be consistent and permit ofaa temporal-causal interpretation, there must be
no cycles F

F..., F= F,, and F pp+11
b
must always be "earlier" than F 1+b (q = 1, 2, ..., p).
€) Finally, n functions f f..., fn must be
given. Each of them depends on z + s variables which
take on the values
1, 2,.., M₁; 1, 2, .… Moi ...; 1, 2, ..., M₂
1, 2, ..., N₁3 1, 2, ..., N₂3 ...; 1, 2,'.., Ng
respectively. These functions are real-valued and
f₁ + f2 +... + f=
holds identically. If in the course of a play which has
been completed the results of the z "draws" and s
"steps" were X, X ..., * J Jo ..., Jg respectively, (x = 1, 2, ..., M ,= 1, 2,..., N3
μ = 1, 2, *.*, Z, V = 1, 2, •*., s) the players S
S, •.., obtain from each other the amounts
f(x ..., ., yg), f2(x₁, ..*, X У, ..., У)
... )x.., X2 Y..., g)
(On closer inspection it can be seen that in spite of this somewhat lengthy description we are dealing here with quite simple matters.
Actually, in several respects our definition might have been somewhat more

The identity

f1 + f2 + ... + f = 0
expresses that the players make payments to each other only, but collectively they neither gain nor lose.

general. (1) We(H) could, e.g., have included the case that the M, N and
a, a2... adepend M
 on the results of the "earlier" "draws"
and "steps", and the like. It is easy to see, however, that such generalizations would be inessential.)
2. With this definition the concept of a game of strategy is
precisely defined. But it also becomes clear that, as we indicated at the
beginning of 1., the expression "S tries to achieve a result as advantageous as possible" is rather obscure. What constitutes the most advantageous result for the player S is obviously the largest possible
value of f, but how can any value of f be "achieved" by S? By
himself, S m is in no position to fix the value of f! The value of fm
depends on the variables x₁, ..., X У, .…, yg, only part of which are
determined by S's decision (viz. those y, for which S has the "step"
F₁, i.e., those for which S(F) = ) All other variables y, depend
on the decisions of the participants and all variables x depend on
chance.
In our case, the "unforeseeable" chance event is actually the
factor which it is easiest to deal with. In fact, assume that a particular
fm depends only on those y, which are decided on by Sm (S(F") = Sm(
and in addition on the x (which are determined by chance). In that case,
S can at least anticipate this much: If I make certain moves, I can expect such and such results (i.e., values of f) with such and such probabilities (since the probabilities aH( αμ), 2)H( αλμ), ..., αν
(H)
are given) -
regardless of how the other players act! If we now assume that "a most
advantageous result" is the highest possible value of the expectation (and
this assumption or a similar one has to be made in order to apply the
methods of the theory of probability)" we have, in principle, solved our
problem. For, we have here a simple maximum problem: The values of those
variables y, which S has to determine must be so chosen by him that
the expected value of fm (which depends only on these variables y,) becomes as large as possible.
It is this type of game which in the theory of probability is
treated in the so-called "theory of games of chance." A typical example
is roulette: Let the number of players be k+ 1 (S, ... S are the
"pointers", Sk+1 1s the "banker"), Sk+1 has no influence whatsoever on
the game,? and the result achieved by Sg, fg, (l = 1, 2, ..., k) only
4 We shall not enter on the well-known objections to the use of the expected value (and the ensuing attempts to replace the latter by the socalled moral expectation or similar concepts). The difficulties that form
the subject of our considerations are of a different nature.
5 Anyway, he has no need to, since according to the rules of the game his
gain after each play is 2.70% of the turn-over.

depends on chance and his own actions.6

The name alone, "game of chance", indicates that the main emphasis is put on the variables x, which are dependent on chance, and not on
the variables y, which are subject to the decisions of the players. But
this is exactly what we are interested in. We shall try to investigate the
effects which the players have on each other, the consequences of the fact
(so typical of all social happenings!) that each player influences the results of all other players, even though he is only interested in his own.
$1. GENERAL SIMPLIFICATIONS
1. The definition of a game of strategy given in the Introduсtion is rather complicated, which may appear justified in view of the fact
that games of strategy may be arbitrarily complex. Nevertheless it is
possible to bring all games falling under this definition into a much
simpler normal form, in a way, into the simplest form that is at all conceivable. We contend that it is sufficient to consider games of the
following kind:
z = 1 (i.e., only one "draw" takes place).
s = n, the v-th "step" being that of the player S. (5(F,) = S,).
The relation "earlier" is never realized (i.e., each player must
make his dispositions without knowing anything about the other participants
or about the "draw").
The play thus takes the following course: Each player Sm
(m = 1, 2, ..., n) chooses a number 1, 2,..., Nm without knowing the
choices of the others. Now a "draw" takes place in which the numbers
1, 2,..., M will appear with the probabilities .., αM The
results achieved by the players are (if the "draw" and the n "steps" have
resulted in x, 1, J2 •.*, n
f,(X, J, ..•, , f(x, 1 .., ,... f(*,V •.., n
This apparently far-reaching restriction is actually not essential,
Let the "steps" of the players Sm (S(F.) = 3m) be those with
for the following reason:
the numbers (m(, vm(, ..., (m). Obviously, it is inadmissible to make
m
the assumption that Sm might be able to tell, before the start of the
6 As may be guessed, on the basis of the preceding footnote, the unambiguous result in this case for the behavior of the pointers is rather
trivial: if possible they should have the turn-over zero; the closer they
approximate it, so much the better!

game, what his choices for all these steps are going to be. It would mean
a restriction of his free will and change his chances (for the worse).
For, S's decision in each of these "steps" will generally be significantly
influenced by the results of the "draws" and "steps" known to him at the
moment of his decision.
On the other hand, it may well be assumed that before the play
has started he knows how to answer the following question: What will be
the outcome of the v-th "step" (k = 1, 2, ..,) provided the re- (m)+
k )m( sults of all "draws" and "steps" "earlier" than v(m) are available? In
other words, the player knows beforehand how he is going to act in a precisely defined situation: he enters the play with a theory worked out in
detail. Even if this may not be the case for a particular player, it is
clear that such an assumption will certainly not spoil his chances.
Accordingly, we define the "strategy" of a player S as
follows:
In order to describe completely the "strategy" of a
player Sm (m = 1, 2, ..., n) the following specifications are necessary:

As before, let S have the "steps" with the numbers m
)m()m( ...,(m) and assume that at the moment when
m
(m)
decides on the vm)-th "step" (k = 1, 2,..., ) the
results of the "draws" and "steps" with the numbers
-(m,k) -(m, k) -(m,k) and ana (m,k) (m,k)
m,k
(m,k)
he
respectively are available to him, that is, they Pm,k
are "earlier" than v(m).( For each possible combination
of results of the "draws" and "steps" mentioned above
(obviously, there is only a finite number of such combinations) it must be specified what S's decision
with respect to the vm)-th "step" is going to be (i.e., K
what will be the outcome of this step).
One sees immediately that only a finite number of strategies is
available to S, which we shall call s(m), sm), ..., s(m).
It can now easily be shown (using the assumption on the absence
of cycles in the Introduction, 1., definition of a game of strategy, (8))
that the course of a play is described in a permissible and unambiguous
manner if we specify

1. which strategies 5(1), g(2), ..., 5(n) are being
used by the players S, S, •••S respectively,
2. what are the results of the "draws" E₁, E₂,..., E₂.
Two points should be noted here. In the first place, it is inherent in the concept of "strategy" that all the information about the
actions of the participants and the outcome of "draws" a player is able to
obtain or to infer is already incorporated in the "strategy." Consequently,
each player must choose his strategy in complete ignorance of the choices
of the rest of the players and of the results of the "draws."
Secondly, it has become entirely immaterial that the "draws'
E, E2,..., En are separate events (where for E, = 1, 2, ..., Z,
the numbers 1, 2,M will occur with the respective probabilities
(1) a), a(e),..., a ) since the players must act, i.e., choose their
"strategies" without knowing the outcome of the "draws." But if this is
the case nothing prevents us from combining all z "draws" into a single
"draw", H, the outcome of which will be the aggregates of numbers
x
X ... X (x= 1, 2, ..., M H= 1, 2, ..., z(
with their respective probabilities aa2 .az, or, what
amounts to the same thing, the numbers 1, 2, ..., M (M = M, M₂... M)
with their respective probabilities, which we shall call B₁, B..., BW
Thus we can modify 2. in the following way:
2'. The result of the "draw" H must be specified.
(H may have the results 1, 2, ..., M with the
respective probabilities B₁, B2 ..., PM•)
The choices (interpreted as "steps") of the players S₁,S...
S in 1. together with the "draw" in 2'. are fully equivalent to the
original game (if one takes the fact into account that each "step" 1s
taken in complete ignorance of all other circumstances), and they evidently
constitute a game ' which, indeed, is of the simple form mentioned at
the beginning of this section.
2. The last element to be eliminated from the game, since from
our point of view it is inessential, is the "draw". This is done by replacing the actual results for the individual players by their expected
values. To be exact:
If the players S₁, S2, ...,n have chosen the "strategies"

و(1" )," , .... ) - 1, 2, ..., ೩ ಹ - 1, 2, ..., 1)
and if the outcome of the "draw" H has been the number v ( = 1,2,...
M), then let the results for the players S, S, •.., 3 be
f₁(v, u₁,..., ), f(V, U₁,..,) ... fn(v, u ..., )
(We may disregard the fact that we are dealing with "strategies" and not
with actual "steps" and simply speak of the choices u, U..., .(
If only the choices of u,, u2, •*છ પૂ are kno n, but not yet the
"draw" v, the expected values of f₁, f2....., fn will be
M
Bf(v, u₁, ..., ( )m = 1,2, ..., n)
V1
(f₁ + f +... + fn = 0 implies g1 + 2 +... + gn = 0). It is entirely
in the spirit of the probabilistic method to discount the "draw" altogether and to deal exclusively with the expected values 81, 82 gn
In doing so we obtain the following basic type of a game of strategy which
is even more schematized and simplified.
Each of the players S, S•••, S chooses a number,
S choosing one of the numbers 1, 2, ..., E
Σ 7 (m -
1, 2,..., n). Each player must make his decision without
being informed about the choices of the other participants.
After having made their choices x₁, X... x
(X = 1, 2, ..., , m = 1, 2, ..., n) the players receive the following amounts respectively:
g1(x1,..., X), 2(x... (..., g(x1... )
(where identically g, + 82 +... + gn = 0).
The rules of the game have thus been obtained in a form which
retains only those characteristics of a game of strategy which are essential
to our consideration - and as we have just shown, essentially without loss
In addition, we could also make all equal to each other by assuming
a Σ which is not smaller than any Ση and subdividing each Σ-th case
1η Σ - Ση + 1 subcases, each of which would have the same effect as the
original. But this simplification is inessential.

of generality. Nothing is left of a "game of chance". The actions of the
players determine the result completely (since everything takes place as
if each of the players has his eye on the expected value only). As a result, the feature which was emphasized at the end of the Introduction
emerges
.x...
in a particularly clear form: each Em depends on all x,, x2
The standard case of probability theory that Em depends on x
only (which, of course, cannot hold for all m) now appears to be entirely
trivial.
$2. THE CASE n = 2
1. Since we cannot proceed any further with the same generality,
it is now appropriate to consider the simplest case for n. The case
n = 0 1s meaningless, and so is the case n = 1 (since 81 + g +... +
En = 0); neither involves an actual game of strategy. So we shall now
investigate the case n = 2.
Since 81 + 82 = 0, we can put 81 = 8, 82 =- g. The description of a general two-person game is then as follows:
The players S₁, S2 choose arbitrary numbers among
the numbers 1, 2, ..., Σ. and 1, 2, ..., Σο
respectively, each one without knowing the choice of
the other. After having chosen the numbers x and y
respectively, they receive the sums g(x, y) and
- g(x, y) respectively.
g(x, y) may be any function (defined for x = 1, 2,...,E
y = 1, 2, ..., !).
It is easy to picture the forces struggling with each other in
such a two-person game. The value of g(x, y) is being tugged at from two
sides, by S₁ who wants to maximize it, and by S₂ who wants to minimize
it. S₁ controls the variable x, S2 the variable y. What will happen?
2. After S, has chosen the number x (x = 1, 2, ..., E),
his result g(x, y) still depends on the choice y of S, but in any
event g(x, y) Miny g(x, y). And by an appropriate choice of x this
lower limit can be made equal to Maxx Min, g(x, y) (and not any larger!).
I.e., if S, so wishes, he certainly can make g(x, y)
> Max, Min,g(x, y)
(irrespective of what S₂ does!). The same argument holds for S. If

S so wishes, he certainly can make g(x, у)
< Min Max g(x, y)
(irrespective of what S₁ does!).
If now
Max Ming(x, y) = Min Maxxg(x, y) = M
it follows from the above, as well as from the fact that S, wants to maximize g(x, y) and S₂ wants to minimize it, that g(x, y) will have the
value M. For, S, is interested in making it large and can keep it from
becoming smaller than M. S₂, on the other hand, is interested in making
it small and can keep it from becoming larger than M. Hence it will have
the value M.
Though, in general,
Max Min g(x, y) Min Maxx8(x, y)
it is not at all true that the = sign always holds. Actually, it is
easy to exhibit such g(x, y) for which the sign holds, that is, for
which the above consideration breaks down. The simplest example of this
kind is the following:
2 = 22 = 2, g(1, 1) =1 , g(1, 2) = - 1,
g(2, 1) = - 1, g(2, 2) = 1.
(Evidently, Max Min =- 1 and Min Max = 1.)
Another example is the so-called game of "Morra":8
Σ₁ = Σ = 3, g(1, 1) = 0, g(1, 2) = 1, g(1, 3) = - 1,
g(2, 1) = - 1, g(2, 2) = 0, g(2, 3) =1,
g(3, 1) = 1, g(3, 2) = - 1, g(3, 3) = 0
(Here, too, Max Min =-1 and Min Max = 1.)
The fact that this difficulty comes up can also be realized in
the following way:

Maxx Miny g(x, y) is the best result that S
can achieve if
Also called "gangster baccarat." In the usual formulation, 1, 2, 3 are
called "Paper", "Stone", "Scissors" ("Paper covers the stone, scissors cut
the paper, stone grinds the scissors").

he is "found out" by S; if whenever S, plays x, So plays a y
such that g(x, y) = Min g(x, y). (According to the rules of the game
S was not supposed to know how S, was going to play, he would have to
infer it in some other way. This is what we mean to indicate by the expression "finding out". In the same way, the best result that S₂ can
achieve if S, has found him out is Min Maxx g(x, y). If the two
numbers are equal this means: it makes no difference which of the two
players is the better psychologist, the game is so insensitive that the
result is always the same. It is obvious that this is not the case for
the two games just mentioned: here, everything depends on finding the adversary out, on guessing whether he is going to choose 1 or 2 (or
or 2 or 3).

The fact that the two quantities Max Min and Min Max are
different means that it is impossible for each of the two players, S, and
S₂, to be cleverer than the other.
3. Still, it is possible, by use of an artifice, to force the
equality of the two above-mentioned expressions.
To this purpose, the possibilities of action for the two players
S, and S₂ are extended as follows: At the beginning of the game, s,
is not asked to choose one of the numbers 1, 2, ..., 2₁. He only has to
specify ₁ probabilities
gl gr ... g, (8, 20, 2> 0, ... 2 0, 1 + 2+... + g, =1)

and then draw the numbers 1, 2, ..., E from an urn containing these
numbers with the probabilities & ... s. He then chooses the
number drawn. This may look like a restriction of his free will: it is not
he who determines x. But this is not so. Because if he really wants to
get a particular x, he can specify sx =1, 5 = 0 (for u x). On the
other hand, he is protected against his adversary "finding him out"; for,
if, e.g., 51= 52= 1/2, nobody (not even he himself!) can predict whether
he
is going to choose 1 or 2!
S2 is supposed to act in the same way. He also chooses
probabilities า1, ใ2, •2 and proceeds accordingly.
Let us denote the sequence .., , by and the
sequence n1 2**2 by n. If S, chooses and S2 chooses
1, S, has the expected value
{{ 8(p, a) pa
p=1 q=1

and S has the expected value - h(5, n). The new function h(,n)
includes the old one, g(x, y), in the following sense: if sx = ny =

and 5u=ny = 0 (for u x, v 4 y), then h(5, n) = g(x, у).
We can now apply the same consideration to h(5, n) we applied
to g(x, y). If S, has made the choice t, his expected value is at
least Minh(, n). Hence, he is in a position to obtain the minimal expected value Max Min h(, n) (irrespective of what S2 does!). In
the same way, S₂ can keep the expected value of S₁ from exceeding
the maximal value Min Maxg h(t, n). Again we have
Max Min h(t, n) Min Maxh(t, n)
and the question is whether the equality sign always holds.
Evidently, in this case our chances are better than they were for
g(x, y); for, g(x, y) could be any function, whereas h(, n) is a bilinear form! Even though h(5, n) is essentially a generalization of
g(x, y), yet it is a function of a much simpler type than g(x, y). In
fact, we shall prove in Section 3 that the relation
Max Minh(t, n) = Min Maxh(, n)
holds for all bilinear forms h(t, n) (where Max: is taken for all ; fo
which 1≥o, ..., , O, 1 + .. + = 1, and Min 1s taken for

all for which n2 0, .., 5 2 0, , * ... + 5 1).
4. Anticipating the result we put
Max Min h(t, n) = Min Maxh(&, n) = M
Let be the set of all for which Min h(, n) assumes its maximal
value M, and let be the set of all n for which Max. h(5, n)
assumes its minimal value M. From these definitions the relations below
follow immediately.
(1) If & belongs to A, then always h(E, n) M
(2) Ifn belongs to , then always h(, n) M
(3) If does not belong to a, there exists an
ก for which h(t, n) <M
(4) If n does not belong to , there exists a
for which h(, n) > M
(5) If belongs to and n belongs to ,
then h(g, n) = м.

On the basis of the relations (1) to (5) the following statement
seems justified:
Clearly, S, must choose a complex belonging to
, and S2 must choose a complex n belonging to .
For any such choice, a play has the value M or - M for
S, and S₂ respectively.
Evidently, a two-person game can be called "fair" if M = 0; and
it can be called "symmetric" if the players S₁, S2 have the same roles.
I.e., if on interchanging 5 and n (which presupposes that 21 = 32(
h(5, n) and - h(5, n) are also interchanged, in other words, if
or, equivalently,
h(5, n) = - h(n, 5)
g(x, y) = - g(y, x)
i.e., if the bilinear form h(5, n), or else the matrix g(x, y) is
skew-symmetric. In this case, the game is, of course, also "fair", as can
be seen as follows:
- Max Min,h(, n) = Min Max, - h(E, n) = Min, Maxh(n E)
= Min, Max h(, n)
i.e.,
- M = M, M = 0.9
One can easily see that in our two examples (in 2.) M = 0 since
M contains only $1 = 52 = 1/2 and $1 = $2 = 53 = 1/3 respectively,
and contains only n₁ = 12 = 1/2 and n1 = 2 = n3 = 1/3 respectively.
I.e., both games are "fair" ("Paper, Scissors, Stone" is even symmetric),
and in both examples each player must choose all numbers at random, all of

Use is made of the fact that Max Min = Min Max, i.e., we have applied
our rather deep theorem on bilinear forms. Trivially -- i.e., from
Max Min < Min Max -- it would only follow that
Max Min 0, Min Max ≥0
While this paper was put into its final form, I learned of the note of
E. Borel in the Comptes Rendus of Jan. 10, 1927 ("Sur les systèmes de
formes linéaires...et la théorie du jeu," pp. 52-55). Borel formulates the
question of bilinear forms for a symmetric two-person game and states that
no examples for Max Min< Min Max are known.
Our result above answers his question.

them with the same probability•
von NEUMANN
The following point should be emphasized: Although in Section 1
chance was eliminated from the games of strategy under consideration (by
introducing expected values and eliminating "draws"), it has now made a
spontaneous reappearance. Even if the rules of the game do not contain any
elements of "hazard" (i.e., no draws from urns) as e.g., the two examples in 2.- in specifying the rules of behavior for the players it becomes imperative to reconsider the element of "hazard". The dependence on
chance (the "statistical" element) is such an intrinsic part of the game
itself (if not of the world) that there is no need to introduce it
artificially by way of the rules of the game: even if the formal rules
contain no trace of it, it still will assert itself.
$3. PROOF OF THE THEOREM "Max Min = Min Maх"
1. Let us slightly change our notation by replacing 2₁ by
M + 1, E by N + 1 and g(p, q) by apq We then have
M+1 N+1
n(e, n)- {{ aраа p=1 q=1
Because of the conditions
+..+5 1+BM + BM =1,1 + ... +  += 1+N 1
the complex is already determined by 51 ..M and so is the complex
by, ..., Thus
MN M N
h(5, n) =K
p=1 q=1 q=1
(There is no need to specify explicitly the coefficients in terms of apa a.
We shall make use of only some of the properties of h(t, n) and investigate continuous functions of two sets of variables f(5, n) which satisfy
the following conditions:
(K). If f(g', n) ≥ A, f(5", n) 2 A, then f(, n) ≥ A for
every 0≤OS1,E = 05'+ (1-)g" (i.e., = + (1-),
p= 1, 2, ..., M). If f(5, n') < A, f(5, n") <A, then f(, )<A
for every 0 SG S1,4 = n' + (1 - )n" (i.e., na = ond + (1- (nd
q= 1, 2, ..., N).

(It is clear that h(5, n), being linear in the as well as
in the n, has the property (K).) For these functions f(t, n) we are
going to prove that
Max Min (f(5, n) = Min, Maxf(, n)
where Max& is taken over the range ≥ O, .., 5M Z O, $1 +... + 5M ≤1
and Min 1s taken over the range 1120, .., N20, 1 + + NS1.
We can also write
Max, Mах. 2 Max.
M
Min Min Min f(,
N
น(
$120 220

1120 122°

31≤1 1+251 +..+M
Min Min
Min Nu Max

Max MaxE
SM
f(5, n)
1120 7220
ONu
5120 5220
MO
neS1
tu 15 158gtl +lg
2. We introroduce the following notation:
 1 ,... g) Max = f(B1  ... • .. 18
E+•.+ &s1
MSf(1 .. g) = Min
sf(e ... s ..og)
ng20
n₁+...+ng1
s
Clearly, M and Ma eliminate the dependence of f on 5p and ng
respectively. We are going to prove: if f satisfies the condition (K)
(in 1.), then
мм... 1 M²2 ... M²P мРмм²... M³1 м³2... M³a
- м1 м³2 ... м'а м²1 м²2... M³P
Evidently, we need only prove the following two assertions:
a) If f = f(1, ..., g e ..., ng) is continuous and has
the property (K), the same holds for M³r f and M'Sf.

B) If f =f(,..., ..., 1g) 18 continuous and has
the property (K), then
MMSrMs
 f
= MsMMf Mr
We first prove (a). It is sufficient to consider M f; the
same considerations apply to M'3 f.
We have
M f** ,)•r(, ... .... ) - ....  .... 
 )Max = f ,... 5 1 ... g
§20
B+...+5s1
It is obvious that the continuity of f implies that of f*. We still
have to examine the two properties in (K).
First, let
,.,)* :-  ,.. ,...,*(ng) 2 A, f - ... .1s) 2 A
The f* represent maximal values of f on finite intervals. Since f is
continuous, they are actually assumed, say, for and respectively.
=
Then
,..,)f  .., ,... ng) Z A, f(, ... ngA
and since f sasatisfies (K) (we put 1 = + (1 - )1, ..., p-1
5-1 + (1)−₁ and tos + (1 - 0))
f(,.., 5 1, .., 1g) 2 A.
The inequalities
520, 5 + ・.. + 551, 20, 『1 + ... + ≤1
imply directly
5x20, 51+ ... +x≤1•
Hence, a fortiori, for the maximum f*

. A 5 <(,...  5- f)*..1•,
Second, let
)* ,.. - • ,... n)S A, f*(6, ..., -1 ..., n)≤A
In virtue of the maximum property of f* we have
) ,• ..( ng) S A, f ,... ..... 1g) sA
for all for which
5x0, 5₁ + ... + ²51
Since f sasatisfies (K), this again implies
( ., ..• ng) SA
(กา = งๆ; + (า - ง)า, ..•. กธ= งท + (1 - ง)ก). And since this holds
for all mentioned above, we have
 SA) ng ,.. 1 1-5 ... *(,1
This completes the proof of our assertion (a).

3. We will now show that
(ur
 always (i.e., for all 1, ... gt-lc
 ,... "g-1 M  :- t. Ir in f(s,... ng...,
we keep the variables  ... -  ,. sn- fixed, then f, as
a function of 5n g alone, evidently still sasatisfies the condition (K).
It remains for us to prove (writing ,n for S g):
If f(t, n) is a continuous function and if
f(', n) 2 A, f(5", n) 2 A for '< < &" implies
that f(, n) ≥ A, and if f(5, n') < A,
f(5, n") < A for n'≤n≤n" implies that
f(5, 7) < A, then
Max Min f(E, n) = Min Max f(5, n)
(We write a and b for 1 - 51 - ... - 5-1 and 1-11-.. - s-1
respectively.)

The assertion can also be formulated in the following way: There
exists a saddle pointno (0≤ 0 ≤a, 0540 59), i.e., f( n)
assumes its minimum for n = no in 0< n< b and f(t, n) assumes its
maximum for = in 0Sa.
First of all, we evidently have
Max Min f(E, n) < Min Max f(5, n)
and secondly, the assertion just formulated implies that
hence
Max Min f(&, n) > Min f( n) = f) (
Min Max f(E, n) < Max f(5, no) = f(5 o
Max Min f(, n) = Min Max& f(5, n) = f(50, 10)
Our task is now to find a pair o no with the desired properties.
Let be fixed. For which values of n, oSu ≤ b, does
f(E, n) assume its minimum? The answer is easy. Since f is continuous,
this set is closed, and because of the second assumption about f
(f(5, n') < A, f(E, n") A implies f(5, n) <A for all n'Snsn")
it is convex. But the only closed and convex sets of real numbers are
closed intervals. Therefore, this set is a subinterval of the interval
0, b; we call it K'(), K"().
Ifn is fixed, we conclude in the same way that those ,
0<<a, for which f(5, n) assumes its maximum form a closed subinterval of 0, a; we call it L'(n), L"(n).
Evidently, always K'(!) < K"(t) and L'(n) < L"(n). Furthermore, the continuity of f(5, n) implies that K'(), L'(n) and K"(;),
L"(n) are lower and upper semi-continuous functions respectively.10
Let now &* be fixed. We form the set of all E** With the
following property: There exists an n* such that f(E*, n) assumes its
other
10 Let us indicate the proof for K'(t). It will be the same for the
three functions.
If K'(t) = 0, the assertion is trivial since always K'(5) ≥ o. Let
K'(5) > 0. For o <n< K'(5) - c(€ > 0), there exists a 8> 0 such that
f(, n) Min f(, n) + 8, by the definition of K'(g). Hence, if t 13
sufficiently close to t, we still have f(5, n) > Min f(, n) + 8/2 (because both f(t, n) and Min f(t, n) are continuous); i.e., f(5, n)
does not assume its minimum (inn, for 0o< n <b) in on< K' (t) - e.
Therefore K'() > K'(!) - e, and K'(ţ) Is lower semi-continuous as we
have asserted.

minimal value (in o ≤ n ≤ b) at n = n* and f(£, n*) assumes its
maximal value (in 0< <a) at = **. I.e., we form the union of
all intervals L'(n*) < & ** < L"(n*) where n* assumes all values in the
interval K' (*) < n* < K"(5*).
In the interval K'(*) < n* < K"(*) the lower semi-continuous
function 'L'(n*) assumes its minimum and the upper semi-continuous function L"(n*) assumes its maximum. Hence the set of ** contains a
minimal as well as a maximal element. It also contains every intermediate
element ', which can be demonstrated in the following way: If it were
not so, every interval L'(n*), I"(n*) would lie either entirely to the
left or entirely to the right of t', and both kinds would exist (those
belonging to the smallest as well as to the largest **). Since n*
runs over an interval, both kinds of n* would have a common 1limit-point.
Since both I' (n*) < &' and L"(n*) > !' occur arbitrarily close ton'
(and L', L" are lower and upper semi-continuous respectively), it follows
that L'(n') < $', L"(n") ≥ 5'; i.e., ' belongs indeed to one of the
intervals, namely, to L'(n'), L"(n').
Our &** thus form a closed subinterval of 0, a, which we shall
call H'(g*), H"(&*). H'(*) is the minimum of the L'(n*), H"(g*) 1s
the maximum of the L"(n*), for K'(*) < n* < K"(E*). It is easy to
see that H'(*) and H"(*) are again lower and upper semi-continuous
respectively (this is implied by the corresponding properties of K'(*),
K"(*), and L'(n*), L"(n*)).
It remains to find a $ *(0 ≤ * <a) which at the same time is
a §**, 1.e•, a §* for which H(§*) ≤§*≤ H (g*)。
If no such * existed, every interval H'(*), H"(5*) would
lie either entirely to the left or entirely to the right of *, and both
kinds would exist (¿* = a and g* = o). Since * runs over an interval, both kinds of * would have a common limit-point '. Since
arbitrarily close to ' both H'(*) < * and H" (*) > * occur (and
H', H" are lower and upper semi-continuous respectively), it follows that
H'(5') < &', H"(t') > &', i.e., ' belongs indeed to the interval
H'(E'), H"(t').
We have now proved our last assertion (and hence the assertion
(ẞ)), which concludes the proof of our theorem.
$4. THE CASE n = 3
Having dealt in Sections 2 and 3 with the case n = 2 we now
proceed to the next case, n = 3.
Consider a three-person game characterized, according to the

description at the end of Section 1, by three functions 81, 82 83 of
three variables x, y, z (x = 1, 2, .…., 2₁, y = 1, 2, *.* E
z = 1, 2,..., E₂), where identically
81 + 82 + 83 = 0
In the case n = 2 it was possible to strictly determine the
value of a play for each of the players S, and S₂ with the results
value for 3, = Max, Min p=1 q=1 B(p, )q- Max Mn
p=1 q=1 ,(po D(etuD
Ση Σ
value for S -M1n,ΜΧ ΣΣΒ(Ρ αλέμήα - ΜαΧ,Μ1Π ΣΣΒ(Ρ, α)Ερα p=1 g=1 p=1 q=1
where
value for S₁ + value for S2 =0 .
Let us now try in the case n = 3 to compute the values of a
play for the three players S1, S2, S3. Assume these values to be W1, W2
w₂ respectively. For these values to be satisfactory under any conditions
and without further discussion they clearly should have the following
property: No two players must be able, by forming a coalition, to achieve
an expected value exceeding the sum of the "values of a play" assigned to
them. Furthermore, W₁ + W₂ + W₂ = O must hold, since the players make
payments only to each other.
By putting
Max Min
p=1 q=1 r=1
Ση Σ Σ3
p=1 q=1 r=1
Ση Σ2 Σ3
ΜΣΜΑΠΑ ΣΣΣ p=1 b=1 r=1
(8, (pqr) + 82 (pqr)) pq"r = M1,2
(g1 (pqr( + g3 (pqr)) pr"q = M1,3
(82(pqr) + 83 (pqr)) qrp = M2,3

(the pa form a system of probabilities, as do the 3 similarly pr
na and ar p), S, and S2 by forming a coalition can play an ordinary
two-person game against S3, thereby procuring for themselves (in
accordance with the above) the expected value M1.2 The same holds for
S, and S3, and for S2 and S3 regarding the expected values M1,3 M₁
and M2,3 respectively. Hence we must have
W₁ + W2 M1,2 W₁ + W3 2 M1,3 W2 + W3 Z M2,3
W₁ + W₂ + W3 = 0
Clearly, this is possible if and only if
M1,2 + M1,3 + M2,3 ≤0.
As we are going to show in 2., it is always true that
M1,2 + M1,3 + M2,3 ≥0
and it is easy to give examples in which the > sign holds. Such an еxample is provided by the following three-person game:
31 = 32 = £3 = 3. If among the X₁, X2, X3 (i.e.,
among the choices of S, S, S, formerly also denoted by x, y, z) there are two for which x = v,
Χ. = μ, then μ, v forma "true couple". Clearly,
there will be precisely one true couple, or none at
all.
If no "true couple" exists, let g1= 82 = 83 = O.
If there is a "true couple", let it be μ, v and let
the third of the numbers 1, 2, 3 be . Let
8μ = 8γ = 1, 8λ = - 2.
In this game, evidently M1,2 = M1, M1,3 = M2,3 == 2 (any two S,
S, in coalition can choose v and respectively to form a "true
couple" and thus take from the third player, S₂, the amount 2!). Hence
M1,2 + M1,3 + M2,3 = 6> 0.
The reason that in this game any attempt at valuation is bound to
fail is the following: In order to gain the amount 2, it is only necessary for any two of the three players to get together. They are then in
a position to rob the third one without any ado, in spite of the fact that
the rules of the game are strictly symmetrical, i.e., the game is formally

fair.11 The symmetry would imply the value for each player to be zero, but
this is obviously wrong. If two players only want to, they can procure for
themselves a gain of 2. How is this contradiction to be resolved?
2. Let us proceed systematically.

M1,2 + M1,3 + M2,3 ≥0
always holds. For, evidently,
2, 2 33 M,,2- Max Min (g1)dbr( + g)pqr)) par
p=1 q=1 r=11
(according to our theorem on two-person games)
)(p) + (pq )p 123
- Max Min 33 83 (par) pqr
p=1 q=1 r=1
We have to prove, therefore, that
Μaχ Μ1ΠΕ Σ 3(par)g+ Max Μ1" Σ 8(par)
p,q,r p,q,r
+ MaxMin { 8,(pbr("so
p,q,r
11 This shows that our example is anything but a "pathological" game.
Actually, it is a fairly frequent and typical case. Accordingly, we shall
see in Section 4, 3 and Section 5, 1 that it is even the general case of
a three-person game.
12 Intuitively, this is immediately clear. S₂ and S₂ in coalition
against S, can at best obtain M2,3' hence S₁ for himself (against
all others) can at best obtain - M2,3 (because of our theorem on twoperson games). Likewise, S₂ for himself can at best obtain- M1,3
S, and S2 in coalition can at best obtain M₁,2 "Unity is strength,"
i.e.,
- M2,3- M1,3≤M1,2 M1,2 + M1,3 + M2,320.

THEORY OF GAMES OF STRATEGY S
systems.1 e., for all ', ", "
MinE 83(pqr( + Mn" { B2 (pqr)
p,q,r p,q,r
+ Ming 28」 (paren≤이
p,q,r
This is actually the case. For, if we put
we obtain (because of 81 + 82 + 83 = 0)
bd

p,q,r p,q,r P,q,r
We know already that the > sign actually occurs. The case of
equality must, therefore, be regarded as a degenerate limiting case.
Let us assume that the player S₁ makes a claim on a gain of w₁
per play. How can he enforce it? Evidently, in two ways.
First of all, he can try to play alone. Essentially, this
amounts to setting up a two-person game in which he is on one side and
S2 33 (in coalition) on the other. The value of a play is for him
M2,3 This solution is acceptable only if w₁≤- M2,3 Let us, therefore, assume w₁ >- M2,3
Then only the second possibility remains. S, must try to get
S2 or S3 as an ally. In coalition with S2 or S3 he can win the sum
M1,2 or M1,3₁ per play. Since he wants to keep w₁ for himself, he can
offer the sum M1,2 - W1 to S2, or M1,3 - w, to S3 as the price of
the alliance. It is out of the question, however, that S or S will
accept this offer if in coalition with each other they can win more than
(M1,2- W₁) + (M1,3 - w₁) per play, i.e., if
(M1,2 - W₁) + (M1,3 - w₁) < M2,3° W₁ > 1/2 (M1,2 + M1,3 - M2,3)
Hence we can say that S, has no hope at all of satisfying a
claim w1 which is
> - M₂,3 > 1/2 (M1,2 + M1,3 - M₂,3)

The second number is ≥ the first (because M1,2 + M1,3 + M2,32 0),
hence
W₁≤ 1/2 (M1,2 + M1,3 -M2,3) = 1
must hold. In the same way, it can be shown that
W2≤ 1/2 (M₁1,2 + M2,3 - M1,3) = 2
W3 ≤ 1/2 (M,3 + M2,3 - M1,2) = W3
must hold.
These upper limits for the claims w, W, w of the three
players can easily be obtained. If, say, S₁, S2 enter into a coalition,
they can achieve the gain M₁2 = พี, + พีอ (against 3₂). In the same way,
S₁, S3 or S2, S3 can, by entering an alliance, make sure of the gain
M₁3 = ₁+ 2 and M₂,3 = 2 + 3 respectively. Hence, the highest
possible and still completely justified claims of the three players S,
S₂, S₂ in a play are is was 3 respectively•
3. How is this valuation compatible with the impossibility we
found in 1. to make a general valuation? If M1,2 + M1,3 + M2,3 = o, no
difficulty arises. In this case
พ1 = - M2, 3, 2 = - M1,3, W3 = M1,2
i.e., each player can push his claims all by himself, without the help of
another (and in the face of a possible coalition of his opponents). All
three players can satisfy their claims simultaneously, and accordingly
พีๆ + พิอ + ว= 0 •
The situation is different for M₁,2 + M₁,3 + M2,3 > 0. Since
₁> - M2, 3, 2 > - M1,3, W3 > - M1,2
no player alone can satisfy his claim, and since
1t
the
++ = 1/2 (M1,2 + M₁,3 + M2,3)> 0
is impossible for all three of them to obtain their desired gains
same time. But because
W₁ + W₂ = M1,2 1 + 3 = M1,3 2 + W3 = M2,3
at

each pair of players having entered an alliance (to rob the third) is
assured of its success. Both players can completely satisfy their claims,
while the third player will receive only - M2,3, - M1,3- M1,2
respectively per play, and therefore, his gain will fall short of his
justified claim by the amount of 1/2 (M,2 + M1,3 + M2,3).
This can be formulated as follows: Each of the three players
S, S₂, S must endeavor to ally himself with another player. If he
succeeds, he receives per play
1/2 (M1,2 + M1,3- M2,3), 1/2 (M,2 + M2,3 - M₁,3),
1/2 (M1,3 + M2,3 - M₁,2
respectively. If he does not succeed (i.e., if the two others form a
coalition), he receives only
- M₂,3, - M1,3 - M1,2
respectively. Still another way of describing the situation, and possibly
the most concise one, would be the following:

A
a) A play has for the players 31, 2, 3 the
respective "basic values"
= 1/3 (M1,2 + M1,3 - 2M2,3), V2 = 1/3 (M1,2 + M2,3 - 2M1,3),
V3 = 1/3 (M₁,3 + M2,3- 2M1,2
Since v1 + V₂ + V3 = 0, this is a proper valuation.
B) But for each of any two players entering an
alliance against the third there exists the possibility
to win 1/6 D in excess of the above "basic values",
while the third one sustains a loss of 1/3 D (also
in excess of his "basic value"). We have
D = M1,2 + M1,3 + M₂,3 > .13
(The first case, D = M1,2 + M1,3 + M2,3 = 0, can
also be included in this formulation. Here, (a) is
the result, and -- since D= o -- (ẞ) is vacuous.)
Incidentally
V₁ = - M2,3 + 1/3 D, V₂ = - M₁,3 + 1/3 D, V3 = - M1,2 + 1/3 D

This solution shows immediately that the three-person game is
essentially different from a game between two persons. The actual game
strategy of the individual player recedes into the background. It does
not offer anything new since the formation of coalitions (which is bound
to take place) makes the play a two-person game. But the value of a play
for the player does not only depend on the rules of the game. Rather, it
is a question -- at least as soon as D> 0 -- of which of the three
equally possible coalitions S₁, S2; S, S; S S has been formed. A
new element enters, which is entirely foreign to the stereotyped and wellbalanced two-person game: struggle.
of
$5. PRELIMINARY REMARKS ON GAMES FOR n> 3
1. For n> 3 it has not yet been possible to obtain results
general validity. It may be that the best way to proceed will be in
analogy to the cases n = 2, 3 which we have already dealt with. Let
us recapitulate.
n = 2. We define
M = Max& Min { 81)p( .
p,q
An individual play has for the players S₁, S2 the
values M, - M respectively.
n = 3. We define
M1,2 = Max& Min { (g, (par) +g2 (pqr)) pr p,q,r
M1,3 = Max Minn { (8, (pqr) + 83(pqr)) phq p,q,r
M2,3 = Max& Min { (8 (qr( + 83(pqr)) grp
p,q,r
D = M1,2 + M₁,3 + M2,3

The Max and Min are to be taken for all systems of probabilities,
i.e., we require that
all p 20, 2pp = 1; all $pq 20, Epq³pa = 1; etc.
and similarly,
all p ≥0, Ep"p = 1; etc.

D2 0, and we distinguish two cases, viz. D = 0
and D> 0.
D = O. In this case, a play has for the players
S1, S2, S3 the values - M2,3, - M1,3 - M1,2
respectively.
D> o. In this case, a play has for the players
S₁, S2, S3 the "basic values" - M2,3 + 1/3 D,
-1,3 M₁3 + 1/3 D, - M12 + 1/3 D respectively. In
order to obtain the
term has
correct values, however, another
to be added to the "basic values". This is
due to the fact that each of any two players who form
a coalition against the third (no matter which two)
can procure an additional gain of 1/6 D, while the
third player sustains a loss of 1/3 D per
(in excess of his basic value).
play

From this summary it becomes clear that the cases n = 2 and
n = 3 with D = 0 are of the same type. The case n = 3, D > 0, hoWever, as was already established at the end of Section 4, represents a new
type. We shall denote these two types by the terms strictly-determined
and symmetric non-strictly determined respectively, which are selfexplanatory.
Is there any chance of reducing all games of strategy to these
same two types, even if n > 3? Or are new complications to be anticipated?
In particular, the possibility of asymmetrical non-strictly determined
types has to be contemplated, i.e., types for which the significant
possibilities of coalition-formation are not symmetrically distributed
among all players. For n = 3 this possibility cannot arise. Any
possible asymmetries of the rules of the game are entirely absorbed by the
"basic values" of the three players. But all players are equally capable
of forming coalitions, all three coalitions S, S₂ S, S ತ2, 3, are
equally possible. Let us investigate this question somewhat more closely.
2. In order to characterize a general n-person game we introduce the following constants:
Pn=1
Σ
M)= ΚΑΠΑ
d1=1 ΣΣ d=1
)g")d1 ... Pn) + ... + (1 ... P))
Яня
...Pn-k

where 1, H.., Hk are any k distinct numbers among the numbers
1 2 , and 1, V2..., n-k are the remaining numbers (Max is to be taken for all D Σ Ο, ΣΕ, = 1, and
Min for all mp. "py.....P 2 0, EnI 1). Clearly,
n-k n-k
is the sum per play which the coalition of the players
M(H19H2...H S able to obtain from the coalition of the players S
H
Sv ..., S, (since, in fact, the game is a two-person game). n-k
Evidently,
Ο. Our theorem M() =
on two-person games further
implies that M(..H)- M(vi...n-k) Finally, let H₁, *.*, ?
V . V3 P , Pn-k- be three subsets of 1, 2, •, n, comple- mentary to each other. If the players SHS ... SH as well as
Su..., y and Sp., form fixed coalitions, this is a
P1 Pn-k- three-person game, and we have (putting primes on the quantities involved
in this kind of game
MI,2 - M(H1.H
Mi,3 =M(...-k-)-M)A...)
=
M23=M(v*... pn-k-)"- M(H ..., H)
But according to our results for three-person games we have
i.e.,
M,2 + M,3 + M3 2
(H . ...)2M(H...)M(,...(
We recapitulate:
A given n-person game assigns to each subset
Hs H2s•..., H of 1, 2, ..., n aconstant
M(H...H) (that is the sum per play which the
coalition of the players S ,..., 1s able to
obtain from the coalition of the other players). The
system of the constants M(u...,Hy) always sasatisfies

the following three conditions:
1. M( = O
2. M(H...H) + M(V1...,n-k)0 ir H... H
and v₁,..., 'n-k are complementary subsets of
3.
1, 2,..., n.
M
TI H ..., Hk and are disjoint
subsets of 1, 2, ., n.15

for
It is not difficult to prove the converse, i.e., to specify,
each system of numbers M() ( ranging over all 2 subsets of
1, 2, ..., n) satisfying the conditions 1: to 3., a game of strategy for
which the above constants have precisely these values M] We refrain
from discussing here such an example, which is not deep at all.
3. I venture the conjecture that the complex of valuations and
coalitions in a game of strategy is determined by these 2 constants
alone. We have seen that this is true for n = 2, 3; for n> 3 a
2.16
general proof has yet to be found. While in the case n = 2 no coalition
at all is possible and for n = 3 only one type of coalition is conceivable (i.e., "two against one"), the number of possibilities increases
rapidly for n > 3. If n = 4 one must already decide whether a coalition
"three against one" or "two against two" is going to be formed, i.e.,
which alliance offers the best chances to the participants. If n = 4
it is still possible to discuss the principal cases (on the basis of the
Mart) alone!), but a satisfactory general theory is as yet lacking.
If our conjecture is correct, we have brought all games of
strategy into a natural and final normal form. Each system of 2
constants M[w) satisfying the conditions 1. to 3. represents a class
15 Intuitively, this assertion is as clear as the one considered in the
footnote 12, p. 34.

For n = 2
of
M( = 0, M(1) = M, M(2) = - M, M(1,2) = O
and for n = 3
M() = 0, M(1) = - M2,3' M(2) = - M,3' M(3) = - M₁,2' M(1,2) = M1,2'
M(1,3) = M1,3 M(2,3) = M2,3' M(1,2,3) = 0

"tactically equivalent" games of strategy.17
In conclusion I would like to add that a later publication will
contain numerical calculations of some well-known two-person games (Poker,
though with certain schematical simplifications, Baccarat). The agreement of the results with the well-known rules of thumb of the games (e.g.,
proof of the necessity to "bluff" in poker) may be regarded as an
empirical corroboration of the results of our theory.
17 Another possibility of normalizing the M consists of introducing
"basic values" V₁, V2,..., Vn for the players S, So,., S -- in
analogy to the "values" (of a play for n = 2 and the "basic values"
for n = 3. For the values exceeding the v₁ one obtains, of course, the new constants
pin
Appropriately, the Vp are chosen such that
M(1) = M(2) M(n) V+V₂ + ... + = 0
i.e., all players playing for themselves are equally strong, and differences are due only to the various possibilities of coalitions.
(From 1. - 3. it follows easily that the common value of
M(1), M2)..., Mn) So
If it is zero, all M= 0, i.e., after payment of the "basic values" the
play is strictly determined. Hence this common value represents a kind of
measure of how non-strictly determined the game is, i.e., a measure of the
tactical possibilities the game offers.)