### 入力なくなったら終わる系のとり方
let mut input: Option<usize> = sc.read1();
while let Some(n) = input {
    println!("{}", num[n]);
    input = sc.read1();
}


### 2分探索
 - 億マス計算
 > https://arc037.contest.atcoder.jp/tasks/arc037_c
a, bをソートしておく．  
すると一番小さい候補は0，大きい候補はa[N-1]*b[N-1]，この範囲を2分探索．  
各行(a)を一つ決めると，その行のbの中で K / a 以下のbが掛け算した結果K以上になる．  
このbについても2分探索をする．入れ子で2分探索をするイメージ．  
[TODO]なぜ，外側の2分探索の結果が掛け算の積としてあり得ない値にならないよう保証できているのかがよくわからない．  


### 数学
 - Farey Sequence
 > http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2286  
約数が作れるかは，分母と分子が互いに素かどうかと等価．
オイラーのトーシェント関数を使う．euler_phi_vec関数.

 - Prime Factor Prime
 > https://jag2017autumn.contest.atcoder.jp/tasks/jag2017autumn_c  
足りていなかった発想
素数テーブルはsqrt(R)個まででいい．
範囲内の数の配列を小さい素数から順に使って割っていくことで素因数分解を高速に行う．
その際，割るために使っている素数だけインデックスをジャンプしていい．
sqrt(R)より大きい素数でしか割れないものもでてくるかもしれないが，そのような数で2度割り切れることはない．
したがって，範囲内の数配列中の数が上記の処理をしても1より大きかったらその数のカウンタを増やす．

 - 11
 > https://beta.atcoder.jp/contests/abc066/tasks/arc077_b
 基本的には組み合わせの数n+1Ck <- 順列はABとBAを区別して数えるのでこの問題で言う並び替えた列まで数えてしまう．  
 しかし，重複している数al, arが必ず一組存在．このような部分列を余計に数えた組み合わせの数が求まる．  
 

### 10e+7で組み合わせを割る問題
 
 - 経路
 > https://beta.atcoder.jp/contests/abc034/tasks/abc034_c  
単純，詳細はNoteの付箋のページ　TODO: ライブラリ化 

 - いろはちゃんとマス目
 > https://beta.atcoder.jp/contests/abc042/tasks/arc058_b  
各経由点までの組み合わせ　×　各経由点以降の組み合わせ　を足し上げる．　綺麗に書きたい


### BFS

 - People on a line 
 > https://beta.atcoder.jp/contests/arc090/tasks/arc090_b   
 謎のネットワークを構築　矛盾の判定　ネットワーク上で値を勝手に決めていく　矛盾したらダメ  


### DP

#### 最大最小系
 - 異なる素数の和
 > https://yukicoder.me/submissions/244093  
 Nを構成する素数の和の最大数はある素数だけ戻った位置の最大数+1  
 ループの順を逆にすることで一度しかある素数を使わないようになる．

  - 高橋くんの苦悩 
  > https://abc015.contest.atcoder.jp/tasks/abc015_4  
dp[i][j][k] = k番目まで調べた時の合計幅i，使用枚数jの重要度の最大値  
更新: k番目まで調べたけどそれを使わない時が最大 = dp[i][j][k-1] か，  
                          使う時が最大 = dp[i-a[k-1]][j-1][k-1] + b[k-1]　のどちらか

#### yes/no系
 - yukicoder おもりと天秤
 > https://yukicoder.me/submissions/243877  
 ナップザック問題の和を一定にする組み合わせがあるかどうか -> ライブラリ化した．  
 和を合計の半分にできれば釣り合う．


#### 組み合わせ系
 - DPなすごろく
 > http://yukicoder.me/problems/no/44  
 組み合わせの数を求めるDP．遷移の数で更新式が得られる．
 dp[i][j] = i回投げた時，jのマスにいる組み合わせの数


#### 確率系
 - TDP D サイコロ 
 > https://tdpc.contest.atcoder.jp/tasks/tdpc_dice   
 単にライブラリを使ってはいけない場合もある  
 この場合はdが大きいので素因数分解に時間がかかる　素因数が2,3,5以外にでてきてはいけない  
 配るDP　確率は配ると上手くいくことが多いらしい  
 minでごまかすのは，前の試行で既に倍数になっているなら後でも倍数なのでその分の確率は捨ててはいけないため  


### Priority Queue

 - 3N Numbers   
 > https://beta.atcoder.jp/contests/abc062/tasks/arc074_b    
 区間の大きい組み合わせをp-qで維持したまま探索する   
 小さいのからPopするのは簡単にできなそう？（TODO:要調査) だが符号入れ替えて格納すれば大丈夫  


 ### Union-Find

 - バウムテスト
 > http://arc037.contest.atcoder.jp/tasks/arc037_b  
Union-Findと一緒に各木のNode,Edge数を保存，閉路検出に用いる  
Edge数 >= Node数なら閉路あり



## グラフ

### BFS 
 - Shorten Diameter
 > https://beta.atcoder.jp/contests/agc001/tasks/agc001_c
グラフの直径とは，任意の2頂点間の最長経路長．  
その経路上の中心は，直径が偶数なら ある頂点，奇数なら ある辺になる．  
中心を全パターン試し，Kより大きくなる頂点をBFSにより全て数える．  
そのとき，辺からスタートするときは，辺がつないでいる頂点両方を0にセットするといい．  

 - Fight Against Traffic
 > http://codeforces.com/contest/954/problem/D
TODO:両方からbfsすることの意味を考える  
まず全探索でできないかを考える． グラフ上の距離を計算するBFS関数を作った．  

### 2部グラフ
 - 3 Steps
 > https://img.atcoder.jp/code-festival-2017-qualb/editorial.pdf  
奇サイクル(閉路)  
完全グラフ: 任意の2頂点間に辺がある. 辺の数 = N(N-1)/2  
2部グラフでないグラフには，必ず奇サイクルが存在する！  

### 最小全域木
 - Built?
 > https://beta.atcoder.jp/contests/abc065/submissions/2208092  
最小全域木は無向グラフ上での概念．  
辺が重複しても単にコストが小さい方をとればいいのでエッジを追加するときに含めてしまっても問題ない．  
この問題は単にx方向，y方向それぞれの絶対値をコストとする辺が2本あると考えて，どちらも追加．  
そのときに，それぞれの方向でソートして隣り合うもの同士をつなげた辺だけいれとけばいいのは明らか．  


### Dijkstra
 結果のベクタdistanceの要素がi64::max_value()かどうかをチェックする

 - トレジャーハント
 > https://abc035.contest.atcoder.jp/tasks/abc035_d  
有向グラフにおいて，決まった場所に戻ってくるコストは辺を逆向きにしたグラフにおいて最短経路を求めると得られる．
この問題では1からすべてのNode，全てのNodeから1へ戻るコストの和が欲しいのでO((N+M)logN)で計算可能