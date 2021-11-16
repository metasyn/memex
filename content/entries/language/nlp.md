# natural language processing


# advanced nlp notes

Below are some notes from the [CMU CS 11-711 Advanced NLP](http://www.phontron.com/class/anlp2021/index.html) course taught by [Graham Neubig](http://www.phontron.com/) and some papers related to the course.

## text classification

<details>
<summary>
    Data Statements for NLP (Bender and Friedman 2018) -
    <a href="https://aclanthology.org/Q18-1041/">
        link
    </a>
</summary>

* data statements help us address ethical issues of exclusion, over generalization, underexposure while encouraging generalizability and reproducibility - with the intent of creating more ethical science and engineering
* typical vector space representations of lexical semantics pick up biases which get reflected in models, which have real consequences (racism, sexism, etc)
* the paper proposes long form explanation of the dataset in question, in addition to short forms that can be cited by other papers/research
* the paper goes on to propose a schema for a data statement: language tag, prose description, information about variation such as disordered speech, and information about the speaker(s) - age, gender, race/ethnicity, native language, socioeconomic status; all of these things should also be taken about the annotator of the dataset
* information about the speech situation: time an place, modality, scripted vs. spontaneous, synchronicity, intended audience, and so on
* **takeaway**: context of speaker, annotator, and users of an NLP dataset matters, and we should do a better job as a research community to make sure that datasets have data statements to help convey this information for better and more ethical research.

</details>


<details>
<summary>
    The Hitchhiker’s Guide to Testing Statistical Significance in Natural Language Processing - Rotem Dror, Gili Baumer, Segev Shlomov, Roi Reichart (2018) -
    <a href="https://aclanthology.org/P18-1128/">
        link
    </a>
</summary>

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

<details>
<summary>
    RACE: Large-scale ReAding Comprehension Dataset From Examinations (Lai et al. 2017)
    <a href="https://aclanthology.org/D17-1082.pdf">
        link
    </a>
</summary>

* new test dataset with objective metric for reading comprehension
* there is still a substantial gap between automated and human reasoning using RACE
* **takeaway**: use RACE as one of your metrics if you're doing something related to question answering, reading comprehension, or more generalized type of AI task where information synthesis is one of the goals
</details>

<details>
<summary>
    A machine learning approach to predicting psychosis using semantic density and latent content analysis (Rezaii, Walker & Wolff 2019)
    <a href="https://www.nature.com/articles/s41537-019-0077-9">
        link
    </a>
</summary>

* try to detect schizophrenia via language use
* they suggest that you can use this to create a _digital phenotype_
* some similar studies look at semantic coherence, variance in coherence, and specific lexical or semantic markers
* the goal of the paper on some level is to look into how one can create the _digital phenotype_ that helps earlier detection of different types of psychosis, but namely schizophrenia
* "poverty of content" == "low semantic density"
* the paper also uses the terms negative versus positive symptoms. negative symptoms are more generally something missing or lacking: catatonic behavior for example, losing interest or motivation, lack of concentration. in contrast, positive symptoms include hallucinations, hearing voices, delusions. negative symptoms can happen years before an acute schizophrenic episode - this is called the "prodomal" period
* the paper asserts and cites that "low semantic density" is a central feature of this type of psychosis, and may play a useful role in prediction of psychosis.
* auditory hallucinations, a positive symptom, normally occur later in the progression of the psychosis - but the paper tries to proxy this by looking for people that "implicitly talk about voices and sounds"
* _vector unpacking_ is is one of the central methods they use to measure semantic richness
* they also use a technique called _latent content analysis_
* they contrast semantic density with idea density (number of propositions in a set of words) and information value (something implied by the length of a word vector?)

> Our findings indicate that during the prodromal phase of psychosis, the emergence of psychosis was predicted by speech with low levels of semantic density and an increased tendency to talk about voices and sounds. When combined, these two indicators of psychosis enabled the prediction of future psychosis with a high level of accuracy.

* Sample size is 40 participants over 2 years - train on 30, validate on 10
* Lemmatization, POS tagging, then filter to content words
* Look up vectors using Skip gram based Word2Vec word vectors, trained on 25 years of NYT articles
* the meaning of each sentence is the sum of the word vectors, normalized by the magnitude
* Walds X^2 test is used specifically to show that the individual feature of SEMANTIC DENSITY is statistically significant
* They then use classification metrics (f1, precision, recall, accuracy) to show how the model performed as a proxy for the validity of the features they used
* "poverty of speech" didn't have an effect in the study (the count of content words more generally) - whereas "poverty of content" did
* "density of determiners" is also looked at but the effect wasn't significant
* other metrics for semantic density or lexical richness sometimes are affected by the length of the text. they did not find a significant correlation between semantic density and sentence length.
* after shuffling nouns with nouns and verbs with verbs, they no longer found an effect with the walds x^2 test. this means that the semantic density is sensitive to word ordering. this is interesting since they're using word level features, and its a bag of words type of approach.
* they also contrast with "idea density": 'One such alternative measure is idea density, a quantity that can be measured by dividing the number of verbs, adjectives, adverbs, prepositions, and conjunctions in a sentence by the total number of words.' - they found no effect or correlation with semantic density
* another idea "information density" related to vector length was not found to have any effect
* they also had humans rate sentences for semantic density and showed that their semantic density correlated with it, though a weaker correlation
* they also mention that inter-rater reliability of human judgements is somewhat low to begin with also

</details>

Might read:

[Generative and Discriminative Text Classification with Recurrent Neural Networks -
Dani Yogatama, Chris Dyer, Wang Ling, Phil Blunsom (2017)](https://arxiv.org/abs/1703.01898)

[Approximate Nearest Neighbor - Negative Contrastive Learning for Dense Text Retrieval](https://openreview.net/pdf?id=zeFrfgyZln)

[Net-DNF: Effective Deep Modeling of Tabular Data](https://openreview.net/pdf?id=xfmSoxdxFCG)
