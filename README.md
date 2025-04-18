# **<<五民币区块链白皮书>>**  
# 目录  

- <details>
  <summary>1. 总则</summary>

  [1.1. 目的与背景](#11目的与背景)
  [1.2. 名称与简介](#12名称与简介)
  [1.3. 发行量概览](#13发行量概览)
  [1.4. 创世理念](#14创世理念)
  [1.5. 开源与去中心化](#15开源与去中心化)
  </details>

- <details>
  <summary>2. 节点设置</summary>

  [2.1. 节点类型](#21节点概览)
  [2.2. 国储会权威节点](#22国储会权威节点)
  [2.3. 省储会权威节点](#23省储会权威节点)
  [2.4. 省储行权益节点](#24省储行权益节点)
  [2.5. 全节点](#25全节点)
  [2.6. 公民轻节点](#26公民轻节点)
  [2.7. 访客轻节点](#27访客轻节点)
  </details>

- <details>
  <summary>3. 发行与销毁</summary>

  [3.1. 创世发行](#31创世发行)
  [3.2. 省储行创立发行](#32省储行创立发行)
  [3.3. 省储行质押发行](#33省储行质押发行)
  [3.4. 全节点发行](#34全节点发行)
  [3.5. 轻节点发行](#35轻节点发行)
  [3.6. 决议发行](#36决议发行)
  [3.7. 销毁](#37销毁)
  </details>

- <details>
  <summary>4. 费用及利息</summary>

  [4.1. 链上交易费](#41链上交易费)
  [4.2. 链下交易费](#42链下交易费)
  [4.3. 借贷利息](#43借贷利息)
  [4.4. 质押利息](#44质押利息)
  </details>

- <details>
  <summary>5. 投票机制</summary>

  [5.1. 投票系统](#51投票系统)
  [5.2. 拒绝发行或销毁](#52拒绝发行或销毁)
  [5.3. 增减省储会、省储行节点](#53增减省储会-省储行节点)
  </details>

- <details>
  <summary>6. 技术开发</summary>

  [6.1. 核心模块/core](#61核心模块core)
  [6.2. 接口模块/intf](#62接口模块intf)
  [6.3. 网络模块/network](#63#网络模块network)
  [6.4. 安全模块/security](#64网络模块security)
  [6.5. 存储模块/storage](#65储存模块storage)
  [6.6. 工具模块/tool](#66工具模块tool)
  [6.7. 应用模块/app](#67应用模块app)
  [6.8. 桌面前端/desktop](#68桌面前端desktop)
  [6.9. 手机前端/mobile](#69手机前端mobile)
  </details>

****
# 1.总则
## 1.1.目的与背景
* 基于区块链技术用于推动由五民党发起的<<去中心化民主运动>>的主权区块链系统，创立五民币区块链，推广去中心化民主运动，以建立自由民主的中华民族联邦共和国————中华联邦；
* 五民币区块链是一个不受任何机构掌控的主权区块链系统，一个所有人都能自由使用的法定货币系统，一个所有中华民族联邦共和国公民都能参与投票治理的法定区块链系统；
* 五民币区块链是一个结合公权、金融、市场和区块链的综合系统，将民主化运动和建立联邦制共和国与去中心化的区块链系统深度绑定，以此构建一个具有公信、安全、开放、透明、高效的主权区块链系统。

## 1.2.名称与简介  
* 名称：五民币区块链（英文：Wuminbi Blockchain，简称：WMB），原生数字货币：五民币，符号：WMB；
* 单位：常用单位：元（YUAN），小数单位：分（FEN），元最小为1元，分最小为1分，100分等于1元，统一使用分为系统计算单位，统一使用元为前端展示单位；
* 五民币区块链允许开发者在链上构建DApp、NFT、DeFi和DAO组织等，并且要通过跨链桥、链下支付通道等，将五民币与其他数字货币和法定货币、传统银行等支付方式打通，实现跨国界的链上链下实时收付。
* 五民币区块链结构图
![alt text](https://raw.githubusercontent.com/wumindang/WMB/main/文档库/五民币区块链结构图.png)
## 1.3.发行量概览
   |    发行类型      |            发行金额/简述            |         发行方       |    流通状态    |          备注           |
   |:---------------:|:---------------------------------:|:-------------------:|:-------------:|:----------------------:|
   |   创世发行       |   1,443,497,378.00元 = 14.43亿     |        国储会        |    流通        | 国储会持有               |
   |   省储行创立发行  |   144,349,737,800.00元 = 1443.49亿  |       省储行        |    非流通       | 质押钱包永久质押         |
   |   省储行质押发行  |   36,809,183,139.00元 = 368.09亿    |   质押发行智能合约    |    逐年释放     | 质押发行智能合约持有      |
   |   全节点发行     |   9,999,899,000.01元 = 99.99亿      |        全节点        |    逐区块释放   | 写入PoW共识              |
   |   轻节点发行     |   1,571,981,633,622.00元 = 1.57万亿  |   轻节点发行智能合约   |   逐节点释放    | 公民认证智能合约持有      |
   |   决议发行      |       中华联邦建立后，联储会决议        |        国储会         |        -      | 公民轻节点可投票否决发行   |
## 1.4.创世理念
* 先有人类后有国家，是公民建立国家，国家是公民的国家，是公民治理国家，而不是国家统治公民，公民没有爱国的义务；国家政权的建立其基本原则是保护公民的生命权、自由权、财产权、反抗压迫权和选举与被选举权不受任何的非法侵犯，当国家政权无法保证这一基本原则时，公民有权有义务推翻这个政权，建立一个以保障公民生命权、自由权、财产权、反抗压迫权和选举与被选举权为基本原则的政权。
* 中华民族联邦共和国国家名称是基于中华各民族悠久历史与璀璨文化的沉淀，全称为：中华民族联邦共和国，简称为：中华联邦，或中国及中华民国；中华民族联邦共和国是致力于推行五民主义———公民治理国家（民治）、实现民主共和（民主）、保障公民权利（民权）、建设民生社会（民生）、复兴民族文化（民族）———的联邦制共和国。
## 1.5.开源与去中心化
* 五民币区块链及其附属软件采用MIT开源协议，并在GitHub等多个开源平台上完全开放源代码；
* 五民币区块链将构建一套开源协议和架构，使开发者能快速、简易、低成本的开发部署区块链应用；
* 本系统完全去中心化，任何个人、任何组织都可以加入成为去中心化的节点（全节点、轻节点）之一。

****
# 2.节点设置
## 2.1.节点概览
   |  节点  | 国储会权威节点  | 省储会权威节点  | 省储行权益节点 |     全节点     |  公民轻节点  |  访客轻节点 |
   |:-----:|:-------------:|:------------:|:------------:|:-------------:|:-----------:|:---------:|
   |  数量  |    1个         |      43个    |    43个      |     无限      |    无限      |   无限    |
   |  功能  | 国家铸币权、监管 | 省铸币权、监管 |  交易验证     | 铸块、链下通道 |  监督、支付  | 跨国界支付 |

![alt text](https://raw.githubusercontent.com/wumindang/WMB/main/文档库/节点图.png)
## 2.2.国储会权威节点  
* 有1个国储会权威节点，不能增删改，拥有19个管理员，分别对应国家储备委员会的19个委员；
* 使用国储会权威节点软件，且只有一个交易钱包地址，采用N=19，T≥13的门限签名，签名通过则19票同时生效，反之则19票同时否决；
* 国储会权威节点软件拥有发起投票、发行数字五民币、销毁数字五民币、增删改新省储会、增删改新省储行、升级公民轻节点认证发行智能合约和交易钱包收入与支出等功能，并承担数据归档和引导节点；
* 国储会节点发起发行数字五民币、销毁数字五民币、增删改新省储会、增删改新省储行、升级公民轻节点认证发行智能合约流程的，需权威节点间投票（国储会、省储会）,且公民轻节点可投票驳回。
## 2.3.省储会权威节点  
* 有43个初始省储会权威节点，不能增删改，拥有9个管理员，分别对应省储备委员会的9个委员，可新增省储会节点，新增节点可删改；
* 使用省储会权威节点软件，且只有一个交易钱包地址，采用N=9，T≥6的门限签名，签名通过则生效，反之则否决；
* 省储会权威节点能发起投票和销毁数字五民币等流程，并承担数据归档，省储会发起销毁本省储会持有的数字五民币的，仅需本省储会通过即可，但公民轻节点可投票驳回。
## 2.4.省储行权益节点    
* 有43个初始省储行权益节点，不能增删改，拥有9个管理员，分别对应省储行的9个董事会成员，可新增省储行节点，新增节点可删改； 
* 使用省储行权益点软件，采用N=9，T≥6的门限签名，签名通过则生效，反之则否决，并承担数据归档；
* 拥有1个交易钱包地址和1个质押钱包地址，质押钱包只有接收数字五民币的权限，用于质押省储行创立发行的数字五民币。
## 2.5.全节点    
* 全节点是数据归档和新块铸块节点，负责所有新区块的铸造，新区块特指除创世区块以外的所有区块，采用PoW共识机制获得铸块权；  
* 全节点数量不限，部署运行全节点软件的即是全节点，任何组织、任何人均可下载安装全节点软件成为全节点，采用N=5，T≥3的门限签名；
* 全节点可开通运营链下支付通道，并获得所运营的链下支付通道交易的全部手续费。  
## 2.6.公民轻节点 
* 使用WuminApp轻节点软件，并使用联邦公民身份识别码（CIIC）完成轻节点认证，拥有1个交易钱包地址；
* 公民轻节点ID、交易钱包地址和公民身份识别码一对一绑定，完成绑定后可获得投票。  
## 2.7.访客轻节点
* 访客轻节点无需注册，下载安装WuminBi节点App即可使用，没有投票权；
* 访客轻节点可生成N个钱包地址。

****
# 3.发行与销毁
## 3.1.创世发行
* 创世发行1,443,497,378.00元数字五民币（14.43亿元），创世发行量以中共第7次人口普查数据为基准，每一个中共国人发行1.00元数字五民币；
   |    中共第7次人口普查数据   |          创世发行量       |     国储会交易钱包地址（五民币）       |         国储会USDT钱包地址(以太坊)              |
   |:-----------------------:|:-----------------------:|:---------------------------------:|:--------------------------------------------:|
   |    1,443,497,378人      |   1,443,497,378.00元     |  5FHneW46xGXgs5mUiveU4sbTy......  |   0x5f6ffe832d1af30cb732959bcfca9725e82e32b9 |
* 一个人在社会中享有若干权益的同时，同样应尽若干义务，尽了若干义务的，亦应享受若干权益，创世发行寓意每一个中共国人支付1.00元，以支持《五民主义》和《去中心化民主运动》；  
* 创世发行1,443,497,378.00元数字五民币，兑换价格为每0.001USDT兑换1.00元数字五民币，即1USDT=1000.00元数字五民币，预计融资额1,443,497.378USDT（144.34万USDT），由国储会权威节点发行并持有；
* 资金用途
   |          项目                            |     占比    |  预算金额（USDT）  |
   |:--------------------------------------------:|:-----------:|:---------------:|
   |中华联邦国家官网开发               |  0.69%      |     1.0万        |
   |中国五民党官网开发                         |  0.69%      |     1.0万        |
   |五民币区块链开发                       |  13.86%     |     20.00万      |
   |联邦公民身份识别码系统开发                     |   3.46%     |     5.00万       |  
   |联邦公民护照管理系统开发                       |   3.46%     |     5.00万       |
   |WuminApp匿名社交软件开发（含公民轻节点功能）   |   6.93%     |     10.00万      |
   |市场推广与社区建设                              |   13.86%    |    20.00万      |
   |交易所及流动性相关费用                          |   20.78%     |    30.00万      |
   |法律、管理及其他费用                            |   6.93%     |     10.00万     |
   |五民党安全基金                                 |   29.34%    |    42.35万      | 
## 3.2.省储行创立发行
* 生成省储行节点的，即发行该省总人口数x100的数字五民币，各省人口以中共第7次人口普查数据为基准（已做省份调整），共计发行144,349,737,800元；  
   |  序号  |    省份     |     人口数量       |          发行金额           |       简述        |
   |:-----:|:-----------:|:----------------:|:--------------------------:|:-----------------:|
   | **合计** |  **-**   |**1,443,497,378人**|   **144,349,737,800.00元** |  **1443.49亿元**   |
   |  1    |   中枢省     |   10,913,902人   |     1,091,390,200.00元      |    10.91亿元      |  
   |  2    |   岭南省     |   28,157,064人   |     2,815,706,400.00元      |    28.15亿元      |  
   |  3    |   广东省     |  106,012,864人   |     10,601,286,400.00元     |    106.01亿元     |
   |  4    |   广西省     |   50,126,804人   |     5,012,680,400.00元      |    50.12亿元      |
   |  5    |   福建省     |   41,540,086人   |     4,154,008,600.00元      |    41.54亿元      |
   |  6    |   海南省     |   10,081,232人   |     1,008,123,200.00元      |    10.08亿元      |
   |  7    |   云南省     |   46,821,766人   |     4,682,176,600.00元      |    46.82亿元      |
   |  8    |   贵州省     |   38,562,148人   |     3,856,214,800.00元      |    38.56亿元      | 
   |  9    |   湖南省     |   66,444,864人   |     6,644,486,400.00元      |    66.44亿元      |
   |  10   |   江西省     |   45,188,635人   |     4,518,863,500.00元      |    45.18亿元      |
   |  11   |   浙江省     |   64,567,588人   |     6,456,758,800.00元      |    64.56亿元      |
   |  12   |   江苏省     |   84,748,016人   |     8,474,801,600.00元      |    84.74亿元      | 
   |  13   |   山东省     |  101,527,453人   |     10,152,745,300.00元     |    101.52亿元     |  
   |  14   |   山西省     |   34,915,616人   |     3,491,561,600.00元      |    34.91亿元      |  
   |  15   |   河南省     |   99,365,519人   |     9,936,551,900.00元      |    99.36亿元      |
   |  16   |   河北省     |   56,282,021人   |     5,628,202,100.00元      |    56.28亿元      |
   |  17   |   湖北省     |   54,543,553人   |     5,454,355,300.00元      |    54.54亿元      |
   |  18   |   陕西省     |   33,824,101人   |     3,382,410,100.00元      |    33.82亿元      |
   |  19   |   重庆省     |   32,054,159人   |     3,205,415,900.00元      |    32.05亿元      |
   |  20   |   四川省     |   80,310,245人   |     8,031,024,500.00元      |    80.31亿元      |
   |  21   |   甘肃省     |   20,617,465人   |     2,061,746,500.00元      |    20.61亿元      |
   |  22   |   北平省     |   21,893,095人   |     2,189,309,500.00元      |    21.89亿元      |
   |  23   |   海滨省     |   24,720,871人   |     2,472,087,100.00元      |    24.72亿元      |
   |  24   |   松江省     |   24,870,895人   |     2,487,089,500.00元      |    24.87亿元      | 
   |  25   |   龙江省     |   22,780,354人   |     2,278,035,400.00元      |    22.78亿元      |  
   |  26   |   吉林省     |   24,073,453人   |     2,407,345,300.00元      |    24.07亿元      |  
   |  27   |   辽宁省     |   42,591,407人   |     4,259,140,700.00元      |    42.59亿元      |  
   |  28   |   宁夏省     |   7,202,654人    |     720,265,400.00元        |    7.20亿元       |  
   |  29   |   青海省     |   5,030,542人    |     503,054,200.00元        |    5.03亿元       |  
   |  30   |   安徽省     |   61,027,171人   |     6,102,717,100.00元      |    61.02亿元      |  
   |  31   |   台湾省     |   23,561,236人   |     2,356,123,600.00元      |    23.56亿元      |  
   |  32   |   西藏省     |   2,763,853人    |     276,385,300.00元        |    2.76亿元       |  
   |  33   |   新疆省     |   9,880,442人    |     988,044,200.00元        |    9.88亿元       |  
   |  34   |   西康省     |   4,513,098人    |     451,309,800.00元        |    4.51亿元       |  
   |  35   |   阿里省     |   2,627,999人    |     262,799,900.00元        |    2.62亿元       |  
   |  36   |   葱岭省     |   7,833,021人    |     783,302,100.00元        |    7.83亿元       |  
   |  37   |   天山省     |   5,634,164人    |     563,416,400.00元        |    5.63亿元       |  
   |  38   |   河西省     |   4,664,727人    |     466,472,700.00元        |    4.66亿元       |  
   |  39   |   昆仑省     |   893,415人      |     89,341,500.00元         |    0.89亿元       |  
   |  40   |   河套省     |   12,110,780人   |     1,211,078,000.00元      |    12.11亿元      |  
   |  41   |   热河省     |   15,489,562人   |     1,548,956,200.00元      |    15.48亿元      |  
   |  42   |   兴安省     |   3,991,080人    |     399,108,000.00元        |    3.99亿元       |  
   |  43   |   合江省     |   8,738,458人    |     873,845,800.00元        |    8.73亿元       |  
* 各省储行发行量之和可少于144,349,737,800.00元，但不得超过144,349,737,800.00元，省储行创立发行的数字五民币发行后即汇入该省储行质押钱包，并永久质押；    
## 3.3.省储行质押发行
* 各省储行质押的数字五民币，由系统质押发行支付100年的质押利息,由系统以智能合约方式在每个自然年结束后自动发行并存入各个省储行交易钱包；  
* 质押发行的数字五民币归各省储行所有，质押满100年后，即永久停止质押发行，不再支付质押利息；  
* 质押发行的初始年利率为5.0‰，从第一年开始以线性衰减的方式每两年减少0.1‰，以广东省省储行和广西省省储行为例，收益如下：  
   |    质押时间    |    质押利率    |   广东省质押年收益（质押本金：10,601,286,400.00元）  |     广西省质押年收益（质押本金：5，012，680，400.00元）   |
   |:-------------:|:-------------:|:--------------------------------------:|:------------------------------------------:|
   |  第1年    |   5.0‰    |       53,006,432.00元                  |    25,063,402.00元                         | 
   |  第2年    |   5.0‰    |       53,006,432.00元                  |    25,063,402.00元                         |
   |  第3年    |   4.9‰    |       51,946,303.36元                  |    24,562,133.96元                         |
   |  第99年   |   0.1‰    |       1,060,128.64元                   |    501,268.04元                            |
   |  第100年  |   0.1‰    |       1,060,128.64元                   |    501,268.04元                            | 
   |  第101年  |   0.0‰    |       0.00元                           |    0.00元                                  |

## 3.4.全节点发行
* 运行全节点的每铸造一个新区块，系统发行999.99元数字五民币用于奖励该节点，全节点铸块发行为第1个至第9,999,999个区块，共计发行量为9,999,899,000.01元数字五民币；
   | 单个区块发行量  |    可发行的区块总量   |     总发行量          |     简述      |
   |:------------:|:-------------------:|:--------------------:|:-------------:|
   |   999.99元    |      9,999,999个   |  9,999,899,000,01元   |   99.99亿元    |
* 当区块高度超过9,999,999个区块后，即永久停止全节点发行，全节点再铸造新块的将不再获得铸块奖励。  
   |  区块高度   | 第1个区块   |  第2个区块   |  第9,999,999个区块  |   第10,000,000个区块   |
   |:---------:|:-----------:|:------------:|:----------------:|:-------------------:|
   |**发行金额**|  999.99元  |  999.99元  |    999.99元    |       停止发行        | 
## 3.5.轻节点发行
* 使用联邦公民身份识别码完成轻节点认证的，将获得认证奖励，可获得认证奖励的轻节点总量为1,443,497,378个，前14,436,417个认证的轻节点，每个认证的轻节点奖励9999.00元；第14,436,417个之后再认证的轻节点，每个认证的轻节点奖励999.00元；  
   |   阶段   |    认证节点数     |  单节点发行金额  |        总发行量          |      简述       |
   |:-------:|:----------------:|:-------------:|:-----------------------:|:--------------:|
   |未超过    |  14,436,417个    |  9,999.00元   |  144,349,733,583.00元    |  1443.49亿元    |
   |超过后    |  1,429,060,961个 |  999.00元     |  1,427,631,900,039.00元  |  14276.31亿元   |
   |总计     |  1,443,497,378个  |        -     |  1,571,981,633,622.00元  |  15719.81亿元   |
* 所有获得公民轻节点认证发行的奖励，均处于锁定状态，在持有该轻节点的公民完成《公民素质教育》题库学习合格后，即可解锁奖励资金，公民轻节点发行智能合约自动发放奖励；  
* 达到轻节点发行总量后，再认证的轻节点不再获得奖励，认证发行奖励以先完成认证优先获得，每个公民身份识别码仅能认证一次；  
* 轻节点发行能让更多的公民参与去中心化的民主运动，早日推翻共产党，通过轻节点发行，鼓励更多的公民学习《公民素质教育》题库，以提高公民的认知。  
## 3.6.决议发行
* 成立公民储备委员会后，由联储会决议发行或销毁数字五民币，经联储会联合会议决议通过后，由国储会权威节点发起发行，权威节点间投票发行；  
* 适时发行纸质五民币，纸质五民币用以替换人民币，数字五民币与纸质五民币按1:1兑换，五民币与人民币自由兑换；  
* 纸质五民币面额由1分、5分、10分、50分、1元、5元、10元、20元、50元、100元共10种面额。
## 3.7.销毁
* 国储会权威节点可发起销毁所持有钱包内的五民币，国储会发起销毁流程的，需所有权威节点间投票，且公民轻节点可驳回；
* 省储会权威节点可发起销毁所持有钱包内的五民币，省储会发起销毁流程的，需所有权威节点间投票，且公民轻节点可驳回。
****
# 4.费用及利息
## 4.1.链上交易费
* 链上交易费由1%的基础链上交易费+动态链上交易费组成，动态链上交易费默认为0元，由支付方自行选择金额，基础链上交易费为支付方必须支付的金额；  
* 链上交易手续费的90%用于奖励铸块的全节点，10%支付给国储会权威节点交易钱包用于社区建设，链下交易和纸质五民币的交易手续费不适用于此条规则；  
* 由运营链下支付通道的节点上链结算链下交易的，除该节点所获得的收益需要支付链上交易手续费外，其余已经收取了链下交易手续费的那部分链下交易的金额，上链后将不再收取链上交易手续费。  
## 4.2.链下交易费
* 链下交易手续费由开通运营链下支付通道的全节点自行设定，可设定的范围为0.1%至1%，链下交易费全部用于奖励运营链下支付通道的全节点，链下交易通道由全节点运营；  
* 链下交易手续费由收款用户承担，根据所使用的链下交易通道的费率从收款额中直接扣除。
## 4.3.借贷利息
* 储委会新发行的五民币，由发行决议分配给各省储会，省储会将其持有的五民币借贷给所在地省储行，以获得借贷利息收益；
* 省储行持有的五民币，根据本省储会决议，将其持有的五民币借贷给辖区商业银行，以获得借贷利息收益。  
## 4.4.质押利息
* 各省储行质押的省储行创立发行的数字五民币，系统累计支付100年的质押利息，满100年后即永久停止支付质押利息。 
* ****
# 5.投票机制
## 5.1.投票系统
### 5.1.1.基础规则
* 使用智能合约编码规则投票，自动执行且不可篡改，智能合约明确各类投票事项，所有投票向全链广播，并采用密封投票技术，防止操纵、贿选等；
* 权威节点负责发起投票、配置投票参数和结果发布，权益节点负责投票者身份、投票流程、投票结果验证，全节点负责将投票结果铸入区块链，所有节点均可监督投票；
* 访客轻节点无投票权，国储会权威节点有19票，其他节点每个节点1票，且公民轻节点必须通过公民认证后才拥有投票权；
* 每次投票的Token一次性使用，防止重复投票，且每次投票每个身份只能投一次票，所有投票的选项只有同意、反对或弃权，对于重复投票、无效身份投票的合约自动拒绝。
### 5.1.2.投票流程
* 投票准备：权威节点在区块链上部署智能合约，设定投票内容、投票开始和结束时间等参数，并广播至全链可投票节点；
* 投票执行：投票节点使用私钥签名，投票加密后广播至区块链网络，由验证节点验证投票结果，验证后投票结果写入区块链不可更改；
* 匿名投票：投票内容和投票节点分离存储，采用零知识证明或动态加密技术，确保无法追溯具体投票用户。
### 5.1.3.计票与结果发布
* 计票规则：投票结束后，智能合约自动解密并由验证节点统计投票结果，计票过程公开，所有权威节点、权益节点、全节点和公民轻节点实时同步；
* 结果验证：任何节点均可通过区块链浏览器审计投票记录，验证总数和投票用户数量是否一致，验证节点验证后交由权威节点发布；
### 5.1.4.安全与透明
* 投票结果验证并载入区块链后，其结果不可篡改，所有投票均需通过验证节点的验证，以防止虚假投票结果；
* 系统配置防欺诈功能，自动检测异常行为，并记录所有操作日志，供后续审计和用户审阅；
* 投票规则、投票记录、投票结果完全公开，任何用户均可查阅；投票规则变更的，必须同步更新本白皮书，并向全链广播更新内容。
## 5.2.拒绝发行或销毁
* 非全票通过的联储会联合会议决议，有超过全链65%的可投票轻节点投票否决发行或销毁数字五民币决议的，可否定联储会发行或销毁数字五民币的决议；  
* 满足联储会联合会议全票通过，且所有省储行节点均全票通过的决议，可直接生效，不再进入轻节点投票环节。  
## 5.3.增减省储会、省储行节点
* 国储会权威节点增加/减少新省储会、新省储行节点的，在国储会权威节点和现有省储会、省储行节点间投票，有超过65%的节点投票同意的，即通过，反之则否决。  
* 超过全链65%的可投票轻节点投票否决增加/减少新省储会、新省储行投票的，可否定权威节点的投票结果。  
****
# 6.技术开发
* 后端语言使用Rust，前期采用Substrate框架开发，智能合约使用lnk开发并兼容EVM以太坊，桌面前端使用Tauri框架，手机前端使用Flutter框架；
* 采用模块化开发，开发初期就要做好为后期去框架的准备，并符合长期技术演进，为今后重构中国的政府、金融、通信等领域的应用预留扩展。
## 6.1.核心模块/core
### 6.1.1.区块链/block
* 创世区块写入创世发行、省储行创立发行、省储行质押发行、公民轻节点发行，其中，省储行质押发行和公民轻节点发行的五民币分别存入省储行质押发行智能合约和公民轻节点发行智能合约地址，创世发行和省储行创立发行存入指定钱包地址；
* 写入创世区块的五民宣言declaration of wumin：先有人类后有国家，是公民建立国家，国家是公民的国家，是公民治理国家，而不是国家统治公民，公民没有爱国的义务；国家政权的建立其基本原则是保护公民的生命权、自由权、财产权、反抗压迫权和选举与被选举权不受任何的非法侵犯，当国家政权无法保证这一基本原则时，公民有权有义务推翻这个政权，建立一个以保障公民生命权、自由权、财产权、反抗压迫权和选举与被选举权为基本原则的政权。
* 写入创世区块的五民主义wuminism：中华民族联邦共和国国家名称是基于中华各民族悠久历史与璀璨文化的沉淀，全称为：中华民族联邦共和国，简称为：中华联邦，或中国及中华民国；中华民族联邦共和国是致力于推行五民主义———公民治理国家（民治）、实现民主共和（民主）、保障公民权利（民权）、建设民生社会（民生）、复兴民族文化（民族）———的联邦制共和国"。
* 创世配置文件使用json定义区块链的创世状态，包括区块链名称、ID、网络标识、引导节点、创世状态、节点映射表等信息，其中，创世状态定义创世宣言、创世发行、创世合约，以及持有创世发行五民币的各账户地址和金额。
### 6.1.2.共识/consensus
#### 6.1.2.1.工作量证明/PoW
* 全节点使用PoW工作量证明共识获得铸块权；
* 从第1个-第9,999,999个区块，每个区块发行999.99元五民币，用于奖励铸块的全节点，超过9,999,999个区块后停止发行，写入pow共识代码；
#### 6.1.2.2.权益证明/PoS
* 全链所有交易均由省储行权益节点采用PoS权益验证，超过90%的验证者节点做出验证后即可交由全节点铸造区块，验证者节点的验证权益来自省储行质押钱包的质押；
#### 6.1.2.3.权威证明/PoA
* 含国储会权威节点、省储会权威节点；
### 6.1.3.节点/nodes
#### 6.1.3.1.国储会节点/nrc_poa_node
* 国储会节点ID：nrc_poa_node，与交易钱包地址和PeerID绑定；
* 门限签名：N=19，T≥13
* 使用专用的国储会节点软件，桌面端：Linux；
#### 6.1.3.2.省储会节点/prc_poa_nodes
* 省储会节点ID：prc_poa_nodes_0001至prc_poa_nodes_0043，每个省储会节点ID绑定一个交易钱包地址和一个PeerID；
* 门限签名：N=9，T≥6
* 使用专用的省储会节点软件，桌面端：Linux；
#### 6.1.3.3.省储行节点/prb_pos_nodes
* 省储行节点ID：prb_pos_nodes_0001至prb_pos_nodes_0043，每个省储行节点ID绑定一个交易钱包地址、一个质押钱包地址和一个PeerID；
* 门限签名：N=9，T≥6
* 使用专用的省储行节点软件，桌面端：Linux；
#### 6.1.3.4.全节点/full_pow_nodes
* 门限签名：N=5，T≥3
* 使用通用的全节点软件，桌面端：Linux；
#### 6.1.3.5.公民轻节点/citizen_nodes
* 公民轻节点PeerID、交易钱包地址和公民身份识别码CIIC绑定后拥有投票权，公民身份识别码由联邦公民身份识别码系统提供；
* 使用通用的公民轻节点软件，WuminApp手机软件，iOS和Android，WuminApp是一个集成钱包、投票、视频、匿名社交的软件；
#### 6.1.3.6.访客轻节点/guest_nodes
* 使用通用的访客轻节点软件，WuminBi手机App，iOS和Android，WuminBi是官方轻节点钱包软件；
### 6.1.4.交易/transaction
* 链上手续费：基础手续费1%，必须，付方支付；动态手续费默认0，支付方可自行设置，非必须；
* 链下手续费：由开通链下支付通道的全节点自行设定，0.1%-1%，收方支付，链下支付通道清算；
* 手续费分成：链上手续费的90%由铸块的全节点获得，10%由国储会权威节点获得，链下手续费全部由运营链下支付通过的全节点获得；
* 链下交易上链的，除全节点获得的手续费外，不用再支付链上交易费，例：小李支付100元五民币给小习，使用的是2B全节点开通的链下支付通道，费率1%，那么小习收到99元，2B全节点收到1元，2B关闭通道上链的，那99元不用支付链上交易费，获得的1元需要支付链上交易费；
### 6.1.5.钱包/wallet
* 使用Hierarchical Deterministic Wallet（层次确定性钱包），兼容比特币、以太坊、币安币等；
* 支持使用助记词恢复钱包，首先实现钱包功能；
* 使用 Rust OsRng 生成 32 字节（256 位）的随机字节；
* 使用32随机字节从 BIP-39 中生成24个助记词；
* 使用24个助记词通过PBKDF2（SHA-512，2048 次）生成64字节种子；
* 使用64字节种子通过sr25519生成64字节私钥；
* 使用64字节私钥通过sr25519生成32字节公钥；
* 使用32字节公钥+主网ID（880711，4字节+2字节校验，采用SS58编码生成钱包地址。
#### 6.1.5.1单一地址钱包/only_wallet
* 单一地址钱包：国储会、省储会、公民轻节点使用的钱包，交易钱包，这些节点使用的节点软件只能生成一个交易钱包地址；
#### 6.1.5.2省储行钱包/prb_wallet
* 双地址钱包：省储行使用的钱包，省储行节点软件仅能生成一个交易钱包地址和一个质押钱包地址;
#### 6.1.5.3多地址钱包/hd_wallet
* 不限地址钱包：全节点、访客轻节点使用的交易钱包，全节点和访客轻节点使用的节点软件不限钱包地址数量，实现跨链兼容；
## 6.2.接口模块/intf
### 6.2.1.API
## 6.3.网络模块/network
### 6.3.1.libp2p
* 节点使用PeerID发现网络，每个节点都有唯一的PeerID，通过网络密钥对生成；
* 使用专用节点软件的节点，需将节点的PeerID和节点ID进行绑定，使用专用节点软件的节点有：国储会节点、省储会节点、省储行节点；
## 6.4.安全模块/security
* 使用Blake2-256哈希算法，
* 使用sr25519
## 6.5.存储模块/storage
### 6.5.1.IPFS
### 6.5.2.RocksDB
## 6.6.工具模块/tool
### 6.6.1.cmd
## 6.7.应用模块/app
### 6.7.1.智能合约/smart
#### 6.7.1.1.质押发行合约/stake_issue_smart
* 合约地址：nrc_poa_node_5ContractBonus...
* 持有金额：3680918313900分；
* 智能合约记录每个省储行创立发行的质押资金总额、质押钱包地址和交易钱包地址，并按照利率公式计算（公式详见3.5省储行质押发行），得出该省储行的利息金额，在每年的12月30日，该智能合约自动向各省储行交易钱包支付利息；
#### 6.7.1.2.公民轻节点发行智能合约/citizen_nodes_issue_smart
* 合约地址：nrc_poa_node_5ContractBonus...
* 持有金额：157198163362200分；
* 在公民完成公民认证，并满足解锁条件之后，该智能合约自动向公民发放奖励，奖励金额详见3.5轻节点发行；
#### 6.7.1.3.逻辑功能智能合约/logical_function_smart
* 逻辑功能智能合约只能由国储会权威节点部署和升级；
* 逻辑智能合约是可以升级的智能合约，负责公民轻节点发行智能合约的逻辑功能的具体实现；
* 公民轻节点发行智能合约的相关功能在此逻辑功能智能合约中实现，根据具体规则和要求在区块链上线后升级；
### 6.7.2.去中心化应用/dapp；
## 6.8.桌面前端/desktop
* 所有桌面前端节点软件均使用Linux系统；
### 6.8.1.国储会/dt_nrc_node
* 导入钱包，显示交易钱包余额、节点ID，发送和接收五民币，升级逻辑功能智能合约；
### 6.8.2.省储会/dt_prc_node
* 导入钱包，显示交易钱包余额、节点ID，发送和接收五民币；
### 6.8.3.省储行/dt_prb_node
* 导入钱包，显示交易钱包余额、质押钱包余额、节点ID，发送和接收五民币；
### 6.8.4.全节点/dt_full_node
* 生成钱包、导入钱包，显示交易钱包余额，发送和接收五民币；
* 显示挖矿获得的奖励明细；
## 6.9.手机前端/mobile
### 6.9.1.WuminApp
* 生成钱包、导入钱包、显示余额、发送和接收五民币；
* 绑定钱包地址、PeerID和CIIC码；
* 匿名社交功能，P2P匿名即时通信，参考Signal的群聊和私聊功能；
### 6.9.2.WuminBi
* 生成钱包、导入钱包、显示余额、发送和接收五民币；
****