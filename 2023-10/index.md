class: center
name: title
count: false

# LSRTM 2023-10: FFI

.p60[![Ferris](./images/ferris.svg)]

.me[.grey[*by* **Nicholas Matsakis**]]
.left[.citation[View slides at `https://nikomatsakis.github.io/lsrtm/2023-10`]]

---

# How to participate

.p50[
    ![Moira Rose, I have questions](./images/i-have-questions.gif)
]

---

# How to participate

## Have a question?

* Do not use chime chat!
    * Start a 🧵 in slack channel `#learn-some-rust-this-month-interest`!
    * Better yet, come on video!

---

# How to participate

## Have a question?

## All questions are welcome

.p40[
    ![no deprecatory mocking](./images/no-deprecatory-mocking.gif)
]

---

# FFI

FFI = "Foreign Function Interface"

Ability to call *in-process* between languages

---

# Do you really *want* FFI?

Alternatives:

* HTTP
* Microservices
* IPC -- see [JSII](https://github.com/aws/jsii)

---

# Reasons to use FFI

* Vended libraries

--
* Sharing large data structures, high perf requirements

--
* Testing code

---

# Cautionary tales

![Corro](./images/corro.svg)

---

# Special role for C

* C has historically been the "lingua franca"

---

# 

* C has historically been the "lingua franca"
