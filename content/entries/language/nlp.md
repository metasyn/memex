# natural language processing


# advanced nlp notes

Below are some notes from the [CMU CS 11-711 Advanced NLP](http://www.phontron.com/class/anlp2021/index.html) course taught by [Graham Neubig](http://www.phontron.com/) and some papers related to the course.

## text classification

* [Data Statements for NLP (Bender and Friedman 2018)](https://aclanthology.org/Q18-1041/)

<details>

* data statements help us address ethical issues of exclusion, over generalization, underexposure while encouraging generalizability and reproducibility - with the intent of creating more ethical science and engineering
* typical vector space representations of lexical semantics pick up biases which get reflected in models, which have real consequences (racism, sexism, etc)
* the paper proposes long form explanation of the dataset in question, in addition to short forms that can be cited by other papers/research
* the paper goes on to propose a schema for a data statement: language tag, prose description, information about variation such as disordered speech, and information about the speaker(s) - age, gender, race/ethnicity, native language, socioeconomic status; all of these things should also be taken about the annotator of the dataset
* information about the speech situation: time an place, modality, scripted vs. spontaneous, synchronicity, intended audience, and so on
* **takeaway**: context of speaker, annotator, and users of an NLP dataset matters, and we should do a better job as a research community to make sure that datasets have data statements to help convey this information for better and more ethical research.

</details>

* [The Hitchhiker’s Guide to Testing Statistical Significance in Natural Language Processing - Rotem Dror, Gili Baumer, Segev Shlomov, Roi Reichart (2018)](https://aclanthology.org/P18-1128/)

<details>

* more data, more compute, deeper nets, and better algorithms lead to more emphasis on empirical results across datasets; but we still need to ensure we do statistical testing to ensure that our results are not coincidental
* paper presentation based around presenting two algorithms that beat one versus the other based on application to a particular dataset
* NLP uses special evaluation metrics often - such as BLEU in machine translation; however the paper shows that many metrics are used across ACL 17 papers - F-score, accuracy, precision/recall, BLEU, ROUGE, pearson/spearman correlations, perplexity, meteor, UAS+LAS
* if the test statistic, under the null hypothesis, comes from a known distribution, the test is parametric (in contrast with non-parametric tests) - in order to know, you can test using known tests like shapiro-wilk (to test if normal), kolmogorov-smirnov (to find the distance between an empirical and cumulative reference distribution), anderson-darling (to test if a sample is drawn from a given distribution)
* parametric tests have stronger power
* paired students t-test - measures population means of two sets of measurements, based that samples come from a normal distribution. it can be applied to measures like accuracy, UAS + LAS
* for other metrics like BLEU, F-score - commonly they're treated as non-parametric

non parametric

* non-parametric tests are either sampling-based or sampling-free
* sign test - tests whether matched pair samples are drawn from distributions with equal medians - assuming that data is i.i.d.
* two tailed sign test, McNemar's test - paired nominal observations (binary labels) applied to a 2x2 contingency table. the null hypothesis is that the marginal probability for each outcome (e.g. true/false) is the same for both algorithms - with a reasonable N, equals Chi-Squared with 1 DOF. Cochran's Q test generalizes te McNemar's test to multi-class classification
* wilcoxon signed rank test - used when comparing two matched samples - null hypothesis is that the differences follow a symmetric distribution around zero. absolute values of differences are ranked. then each rank gets a sign according to the sign of the difference; then sum the signed ranks

parametric

* two main methods are permutation/randomization and the paired bootstrap
* pitman's permutation test - estimates test statistic distribution under the null by calculating the values of the statistics under all possible labellings (permutations) of the test set. the (two sided) p-value of the of the test is calculated as the proportion of these permutations where the absolute difference was greater than or equal to the absolute value of the difference in the output of the algorithm.
* paired bootstrap test - approximate randomization of the permutation test - but sampling is done with replacements - the p value is calculated similarly as the permutation test - used in machine translation, text summarization, semantic parsing - less effective for smaller test sets.

test selection

* if the data comes from a known distribution - use a parametric test
    * higher statistical power
* otherwise, if the data size is small, use a bootstrap or randomization test
* otherwise, use a sampling-free non-parametric test

conclusion

* lots of papers in ACL / TACL don't use the correct tests, or don't include statistical testing at all, which is unfortunate and we should change that
* open question: language data is rarely truly independent
* open question: bonferroni correction when reporting k-fold validation / cross validation results is one way to test for significance - i.e. calculate p value for each fold separately, then perform replicability analysis for the dependent datasets

</details>

[Generative and Discriminative Text Classification with Recurrent Neural Networks -
Dani Yogatama, Chris Dyer, Wang Ling, Phil Blunsom (2017)](https://arxiv.org/abs/1703.01898)
