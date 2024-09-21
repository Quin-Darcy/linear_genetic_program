
#### Definitions
---
- An ***objective function*** is defined as a function $f:I^{n}\rightarrow O^{m}$ with $I^{n}$ denoting the input space and $O^{m}$ denoting the output space.
- In practice, an objective function is not known explicitly but only incompletely by a small set of input-output vectors, $T=\{(i, o)| i\in I'\subseteq I^{n}, o\in O'\subseteq O^{m}, f(i)=o\}$, called the ***training set***.
- A ***genetic program*** can then be regarded as a prediction model which approximates a given objective function, based on a training set.
- Training samples, or elements of $T$, are known as ***fitness cases***. 
- A ***programming language*** $\mathcal{L}$ is defined by the user over an ***instruction set*** and a so-called ***terminal set*** which comprise input values, constants, and memory variables.
- The ***genotype space*** $\mathcal{G}$ includes all programs of a certain ***representation (type)*** that can be composed of elements from a programming language $\mathcal{L}$. 
- The ***phenotype space*** $\mathcal{P}$ denotes the set of all mathematical functions $f_{gp}:I^{n}\rightarrow O^{m}$ with $f_{gp}\in\mathcal{P}$ and $gp\in\mathcal{G}$. 
- The ***fitness function*** $\mathcal{F}:\mathcal{P}\rightarrow V$ measures the prediction quality, the ***fitness***, of a phenotype $f_{gp}\in\mathcal{P}$.
- An ***interpreter*** $f_{int}:\mathcal{G}\rightarrow\mathcal{P}$ maps genotypes to phenotypes which is must be performed before executing a genetic program.
- The ***population*** at time $t$ is denoted as $P(t)\subset\mathcal{G}$. 
- Runtime of evolutionary programs is measured in terms of ***generations***. 
- From a random subpopulation $P'\subseteq P(t)$ of $n=|P'|$ individuals, a ***selection operator*** $s:\mathcal{G}^{n}\times\mathcal{P}^{n}\rightarrow\mathcal{G}^{\mu}$ selects $\mu<n$ individuals for variation.
- A ***genetic operator*** or ***variation operator*** $v:\mathcal{G}^{\mu}\rightarrow\mathcal{G}^{\lambda}$ creates $\lambda$ offspring of the $\mu$ selected parents from population $P(t)$. These $\lambda$ new individuals become part of population $P(t+1)$. 
- Instructions are restricted to ***operations*** -- including conditional operations -- that accept a minimum number of constants or memory variables, called ***registers***, and assign the result to another register.
- Code which manipulates registers not having an impact on the program output at the current position is called ***noneffective code*** or ***introns***. 
- An ***absolute program*** includes all instructions including introns, whereas an ***effective program*** contains only the structurally effective instructions.
- The ***(effective) length*** of a program is measured in the number of (effective) instructions it holds.