# Styleguide

The principles in this guide are not mandatory, but are considered good practise and are highly recommended to follow.

> You may find it odd to follow these practises for the benefit of school assignments. Good habits will serve you long term.

_Author_: Viola Söderlund, violaso@kth.se

## $\alpha.$ Write clean code.

![](https://cleancodebase.com/wp-content/uploads/2024/07/1_h4_BugUpRdyoLbRfeHGS7Q.jpg)

_Clean code_ is a term that summarises several good practises:
1. __Avoid hard-coded values.__ Constants and configuration parameters are much more maintenace friendly.
2. __Use descriptive naming.__ Short, or shortened, names are hard to understand. Descriptive nameing increase readability and reduce the need for comments.
3. __Write only meaningful comments.__ If your code requires a lot of small notices, the code is of poor quality. Good code is readable, and thus maintainable.
4. __Single responsibility principle (SRP).__ Functions should only do one thing per function. Otherwise, an error can cause multiple faults. Modularity is maintainability.
5. __Don't repeat yourself (DRY).__ Copy-paste code (douplicate code) is pure magnetism for noob mistakes. It's also more code to maintain. Be consistent in writing functions and libraries.
6. __Follow the standards.__ Do everyone a favour and follow industry standards, paradigm conventions, and language conventions. Your preferenses come last in the larger context.
7. __The hardest thing is to make it work, not to make it pretty.__ Your priority is to create a working solution. Then, ALWAYS refactor the code and optimise your approach.
8. __Commit.__ You can always revert to a previous state, only given that the code has been saved.

## $\beta.$ Debug statements are not for release.

Please, write debug print statements, spagetti code, spagetti comments, but for your own amusement. You might even have out-commented code lines lying around.

__Do not include debug garbage in your final version.__ Always take time to refactor your code, and write good documentation.

## $\gamma.$ Contemplate the meaning of credit in academic work.

According to _academic integrity_, your submissions should contain the information for others to give you credit, likewise should you also always give credit to others when due. Credit should be given in case of citation, assistance, direction, as well as in case of inspiration. The only exception is teacher-given assistance.

However, this is not the standard in the source code industry. Here licenses and copyright dictates when code has to be referenced. In Sweden, all written works are automatically copyrighted, including source code. One cannot refuse this copyright. For example, the license CC0, meant to mimic _public domain_ as defined in the U.S, is not applicable in Sweden. Licenses like MIT, gives others right to distribute, mutate, and use a piece of code however, without the need for giving credit.

Also, everything submitted to KTH for assessment is covered by the Swedish _Principle of public access to official records_ (offentlighetsprincipen), giving any Swedish person right to request and view your code. So, don't do anything stupid.

My recommendation is to always give credit when the effect of the citation, assistance, direction, or inspiration, is significant enough to have an impact on your academic performance.