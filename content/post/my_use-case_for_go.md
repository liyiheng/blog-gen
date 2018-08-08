---
date: 2017-08-28
title: "[译]我的go语言使用场景"
draft: false
categories:
  - golang
tags:
  - golang
---


原文地址https://mariosangiorgio.github.io/post/my-use-case-for-go/ <br>

>After using a few very good applications written in Go (Syncthing, Docker and Hugo are some examples) I wanted to get to learn a bit more about the language.

用了一些go写的优质应用（例如[Syncthing](https://syncthing.net/)、[Docker](https://www.docker.com/)和[Hugo](https://gohugo.io/)）后，我打算更深入地学习一下这门语言。<br><br>
<!--more-->

>I'm very interested in programming languages theory and how it could give developers the tools they need to write software in the best possible way and with as many guarantees as possible on the correctness of the resulting applications.

编程语言的理论以及它如何将以最好的方式编写软件并尽可能保证结果程序正确性的工具提供给开发者很感兴趣，我对此很感兴趣。
 
 >To get an idea of where programming languages theory is headed have a look at the post Graydon Hoare (the creator of Rust and now one of Swift’s developers) published discussing possible new research directions for programming languages.

为了了解编程语言的发展方向，我看了[Graydon Hoare(Rust创造者，现在是Swift开发者之一)](http://twitter.com/graydon_pub)发表的讨论编程语言新研究方向可能性的帖子。<br><br>

>I obviously wasn’t expecting Go to address any of these issues but I find it very interesting that its designers deliberately ignored lots of what I would consider basics features in the name of simplicity. The lack of generics is often mentioned in discussions regarding Go. I would have liked Go to have algebraic data types and immutability by default. I would gladly give nil away to get these features.

显然我没指望go解决这些争议，但我发现有意思的是，它的设计者们以简洁的名义有意忽略了一些我会考虑的[基础特性](http://yager.io/programming/go.html)。[缺少泛型](https://medium.com/@sameer_74231/go-experience-report-for-generics-google-metrics-api-b019d597aaa4)经常在关于go的讨论中被提到。我希望go有代数数据类型、默认不可变。我乐意放弃[nil](https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare)换来这些特性。<br><br>

>On the positive side, Go has good libraries, good tooling, a common style and a syntax that is extremely easy to pick up. It’s fast enough and it has good support for concurrency via goroutines. It also produces executables that are very easy to deploy anywhere.

好的方面，go有不错的库、工具链和统一的风格而且它的语法上手极简单。它速度够快而且通过[goroutine](https://tour.golang.org/concurrency/1)很好地支持并发。它还能生成在哪都能方便部署的的可执行文件。<br><br>

Given this description, it seems to me that Go is an evolution of C and Python and I decided to give it a try rewriting a project originally written in Python I am working on.

鉴于这些描述，在我看来go是C和Python的进化我决定通过重构一个我正在做的之前用Python写的[项目](https://github.com/mariosangiorgio/dunbar-go)来尝试一下它。<br><br>

>That project is very simple, small and self contained. It interacts with Twitter (a good library is a huge plus), and it analyzes my interactions with the people I follow. The goal is to find out if I’m following people I am not interacting with much. I somehow ended up following too many people and I needed to find a way to bring my following to a more manageable number.

那个项目很简单，很小而且是自包含的。它和Twitter交互（不错的库是一大亮点）并分析我和我关注的人之间的互动。目的是从我关注的人中找出互动不多的。不知何故我关注了太多人，我需要想办法把我的关注恢复到一个更易管理的数目。<br><br>

>That is admittedly something too simple to draw conclusions, but I think it was still good enough to get a feeling of the language and I had a good impression. Picking up the language has been very easy, the library I used is good. With respect to Python I have a type system, albeit limited, and I get an executable I can easily deploy with no additional dependencies.

诚然这种事太简单没发做出结论，但我觉得通过这件事感受一下这门语言也挺好，可以有个清晰的印象。这门语言上手简单，我用的库也不错。关于Python，我有一个类型系统，虽然有限，我还是能得到可以不用附加依赖就能轻松部署的可执行文件。<br><br>

>Given this experience I believe I’ll consider Go again for projects of a similar size and scope or for simple command line applications. For that use case, I believe it’s a much better fit than Python while it’s simplicity could make me be more productive than if I were using other languages.

鉴于以上经验，我认为对于类似规模和适用范围的命令行应用我会再次考虑go。在这个应用场景中，我相信go比Python更合适，因为与其他语言相比它能给我带来更高的生产力。<br>


原文地址https://mariosangiorgio.github.io/post/my-use-case-for-go/ <br>
