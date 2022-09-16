# natural language processing

# advanced nlp notes

Below are some notes from the [CMU CS 11-711 Advanced NLP](http://www.phontron.com/class/anlp2021/index.html) course taught by [Graham Neubig](http://www.phontron.com/) and some papers related to the course.

<details>
<summary>
    Data Statements for NLP (Bender and Friedman 2018) -
    <a href="https://aclanthology.org/Q18-1041/">
        link
    </a>
</summary>

- data statements help us address ethical issues of exclusion, over generalization, underexposure while encouraging generalizability and reproducibility - with the intent of creating more ethical science and engineering
- typical vector space representations of lexical semantics pick up biases which get reflected in models, which have real consequences (racism, sexism, etc)
- the paper proposes long form explanation of the dataset in question, in addition to short forms that can be cited by other papers/research
- the paper goes on to propose a schema for a data statement: language tag, prose description, information about variation such as disordered speech, and information about the speaker(s) - age, gender, race/ethnicity, native language, socioeconomic status; all of these things should also be taken about the annotator of the dataset
- information about the speech situation: time an place, modality, scripted vs. spontaneous, synchronicity, intended audience, and so on
- **takeaway**: context of speaker, annotator, and users of an NLP dataset matters, and we should do a better job as a research community to make sure that datasets have data statements to help convey this information for better and more ethical research.

</details>

<details>
<summary>
    The Hitchhiker’s Guide to Testing Statistical Significance in Natural Language Processing - Rotem Dror, Gili Baumer, Segev Shlomov, Roi Reichart (2018) -
    <a href="https://aclanthology.org/P18-1128/">
        link
    </a>
</summary>

- more data, more compute, deeper nets, and better algorithms lead to more emphasis on empirical results across datasets; but we still need to ensure we do statistical testing to ensure that our results are not coincidental
- paper presentation based around presenting two algorithms that beat one versus the other based on application to a particular dataset
- NLP uses special evaluation metrics often - such as BLEU in machine translation; however the paper shows that many metrics are used across ACL 17 papers - F-score, accuracy, precision/recall, BLEU, ROUGE, pearson/spearman correlations, perplexity, meteor, UAS+LAS
- if the test statistic, under the null hypothesis, comes from a known distribution, the test is parametric (in contrast with non-parametric tests) - in order to know, you can test using known tests like shapiro-wilk (to test if normal), kolmogorov-smirnov (to find the distance between an empirical and cumulative reference distribution), anderson-darling (to test if a sample is drawn from a given distribution)
- parametric tests have stronger power
- paired students t-test - measures population means of two sets of measurements, based that samples come from a normal distribution. it can be applied to measures like accuracy, UAS + LAS
- for other metrics like BLEU, F-score - commonly they're treated as non-parametric

non parametric

- non-parametric tests are either sampling-based or sampling-free
- sign test - tests whether matched pair samples are drawn from distributions with equal medians - assuming that data is i.i.d.
- two tailed sign test, McNemar's test - paired nominal observations (binary labels) applied to a 2x2 contingency table. the null hypothesis is that the marginal probability for each outcome (e.g. true/false) is the same for both algorithms - with a reasonable N, equals Chi-Squared with 1 DOF. Cochran's Q test generalizes te McNemar's test to multi-class classification
- wilcoxon signed rank test - used when comparing two matched samples - null hypothesis is that the differences follow a symmetric distribution around zero. absolute values of differences are ranked. then each rank gets a sign according to the sign of the difference; then sum the signed ranks

parametric

- two main methods are permutation/randomization and the paired bootstrap
- pitman's permutation test - estimates test statistic distribution under the null by calculating the values of the statistics under all possible labellings (permutations) of the test set. the (two sided) p-value of the of the test is calculated as the proportion of these permutations where the absolute difference was greater than or equal to the absolute value of the difference in the output of the algorithm.
- paired bootstrap test - approximate randomization of the permutation test - but sampling is done with replacements - the p value is calculated similarly as the permutation test - used in machine translation, text summarization, semantic parsing - less effective for smaller test sets.

test selection

- if the data comes from a known distribution - use a parametric test
  - higher statistical power
- otherwise, if the data size is small, use a bootstrap or randomization test
- otherwise, use a sampling-free non-parametric test

conclusion

- lots of papers in ACL / TACL don't use the correct tests, or don't include statistical testing at all, which is unfortunate and we should change that
- open question: language data is rarely truly independent
- open question: bonferroni correction when reporting k-fold validation / cross validation results is one way to test for significance - i.e. calculate p value for each fold separately, then perform replicability analysis for the dependent datasets

</details>

<details>
<summary>
    RACE: Large-scale ReAding Comprehension Dataset From Examinations (Lai et al. 2017)
    <a href="https://aclanthology.org/D17-1082.pdf">
        link
    </a>
</summary>

- new test dataset with objective metric for reading comprehension
- there is still a substantial gap between automated and human reasoning using RACE
- **takeaway**: use RACE as one of your metrics if you're doing something related to question answering, reading comprehension, or more generalized type of AI task where information synthesis is one of the goals
</details>

<details>
<summary>
    A machine learning approach to predicting psychosis using semantic density and latent content analysis (Rezaii, Walker & Wolff 2019)
    <a href="https://www.nature.com/articles/s41537-019-0077-9">
        link
    </a>
</summary>

introduction

- try to detect schizophrenia via language use via the creation a _digital phenotype_
- the goal of the paper on some level is to look into how one can create the _digital phenotype_ that helps earlier detection of different types of psychosis, but namely schizophrenia
- "poverty of content" == "low semantic density"
- the paper also uses the terms negative versus positive symptoms. negative symptoms are more generally something missing or lacking: catatonic behavior for example, losing interest or motivation, lack of concentration. in contrast, positive symptoms include hallucinations, hearing voices, delusions. negative symptoms can happen years before an acute schizophrenic episode - this is called the "prodromal" period
- the paper asserts and cites that "low semantic density" is a central feature of this type of psychosis, and may play a useful role in prediction of psychosis.
- auditory hallucinations, a positive symptom, normally occur later in the progression of the psychosis

results

> Our findings indicate that during the prodromal phase of psychosis, the emergence of psychosis was predicted by speech with low levels of semantic density and an increased tendency to talk about voices and sounds. When combined, these two indicators of psychosis enabled the prediction of future psychosis with a high level of accuracy.

- sample size: 40 participants
- time frame: 2 years until conversion
- training data: speech samples from 23 who do not "convert", 7 who do not
- holdout/validation data: 5 who convert, 5 who do not
- data are transcriptions of the Structured Interview for Prodromal Symptoms (SIPS)
- methods: **vector unpacking** and **latent content analysis**

methods: vector unpacking

step 1.) create word vectors

- skip-gram word2vec from gensim
- context window: 5
- hidden units: 200
- training data: 25 years of NYT text: 42.8M sentences
- preprocessing: lemmatization

step 2.) create sentence vectors

- preprocess: content words only, POS tag, lemmatize
- sum up individual word vectors by looking them up in the word vectors from step 1
- take l2 norm of the sentence

step 3.) measuring semantic density

- assign weights to individual words
- linearly combine them to approximate the sentence vector
- cost function: euclidean distance
- objective: minimize sum of squared errors
- model: neural network
- architecture: single layer - each dimension of each word vector is connected to the identical dimension of the target sentence - but words are not connected at all to each other.
- pruning: weight == 0 if weight < (iteration / tau \* max_iters)
  (where tau=100, max_iters=500)
- produces roughly 30-50 non zero weights across the lexicon
- lastly, iterativel bisect the top N ranked weights; compare F-ratio of the two groups (the authors dont say how long/far this goes) and select the group with the higher F-ratio
- in the end these are called **meaning vectors**
- in summary: by having access to all the words in the lexicon to reconstruct the original sentence, and having access to (generally) more non-zero weights in practice than there are words in the sentence, we effecctively "unpack the sentence vector" into a larger number of words that represent the sentence, but then use F tests to go backwards back to (some) smaller group of high density (highly weighted) words

information value

- average vector length (norm) of a word as a measure of semantic density
- vector length is suggested a potential proxy to true semantic density

semantic density

- density = len(meaning vectors) / len(tokens in sentence)
- mean density = sum(sentence vectors) / len(sentence vectors)
- alternatives: Information value (vector length) and Idea density (measurement of content word usage) did not have effects and did not correlate with mechanical turk responses they got; but semantic density did (weakly, but significantly) - iter-rater human reliability may also be low though

latent content analysis

- represent particiapnts sentences as vectors again, normalized
- semantic probes: top 13.5K most commonly written english words (from NYT)
- find closest probe word via cosine similarity between each sentence
- averaged probe words for convertered/non-convertered
- determine each probe words "base rate" cosine - find the degree to which that word is considered "similar" to some other set of sentences that we consider is "normal" in contrast with the text from the converters and non-converters
- dataset: 30k users on reddit, 30-100 posts in close proximity in the same subreddit - 400M words
- preprocessing: sentence segementation, POS tagging, sum word vectors (using the same word vectors trained from NYT)
- tf-idf weight the probe words for each group, keep the top 50
- reduce dimensionality of top 50 probe words from 200 -> 2 with t-SNE
- kmeans++, determine k=14 by maximizing silhouette coefficient
- used this approach to find a cluster around auditory hallucinations, voices, sounds and other auditory perceptions
- the VOICES cluster's items were then summed, normalized, and turned into a predictor variable by measuring the largest cosine between the participants sentences and the CONVERSION target

model: logistic regression with semantic density feature

- feature analysis test: Wald's Chi-Squared test
- Semantic Density was a strong predictor of conversion
- Poverty of Content (or semantic density) had more predictive power than Poverty of Speech (# of content words used)
- Word order randomization destroyed effect (while preserving sentence length and POS)

model: logistic regression with VOICES cluster similarity feature

- feature analysis test: Wald's Chi-Squared test
- Voices was a strong predictor of conversion
- some work on ensuring the structure of the interview didn't cause the closeness to the VOICES cluster (since interlocutor speech was separated in the data processing)

model: logistic regression with both

- (Precision = 1; F1 score = 0.89, Sensitivity/Recall = 0.80, Specificity = 1)
- VOICES aligns with positive symptoms
- low semantic density with negative symptoms
- the two features are not correlated

technologies:

- gensim - word2vec implementation
- tensorflow - word2vec implementation
- stanford corenlp server
- stanford PCFG for pos tagging
- stanford parser for sentence tokenziation
- NLTK WordNetLemmatizier

summary:

> In future studies, larger cohorts of patients, more variety in the neuropsychiatric disorders under investigation, and the inclusion of healthy controls could help clarify the generalizability and reliability of the results. Further research could also investigate the ways in which machine learning can extract and magnify the signs of mental illness. Such efforts could lead to not only an earlier detection of mental illness, but also a deeper understanding of the mechanism by which these disorders are caused.

questions

- usage of F ratio in meaning vector selection?
- semantic density as a property of a set of words: what in their model would have made it so sensitive to word ordering - since it is kind of bag of words?
- other metrics for semantic density or lexical richness sometimes are affected by the length of the text. they did not find a significant correlation between semantic density and sentence length?

</details>

Might read:

[Generative and Discriminative Text Classification with Recurrent Neural Networks -
Dani Yogatama, Chris Dyer, Wang Ling, Phil Blunsom (2017)](https://arxiv.org/abs/1703.01898)
[Approximate Nearest Neighbor - Negative Contrastive Learning for Dense Text Retrieval](https://openreview.net/pdf?id=zeFrfgyZln)
[Net-DNF: Effective Deep Modeling of Tabular Data](https://openreview.net/pdf?id=xfmSoxdxFCG)
