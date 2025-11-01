# Common used structures for competetive programming in RUST

### Примеры

В файле [segtree.rs](segtree.rs) реализовано дерево отрезков для произвольной функции (min, max, etc.)

```bash
$ rustc segtree.rs
$ ./segtree
10 2
1 2 3 4 5 6 7 8 9 10
0 1 2   // Запрос set в @1 значение @2
1 2 6   // Запрос get в интервале [@1; @2)
```
