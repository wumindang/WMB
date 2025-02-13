**<<五民币金融区块链系统白皮书>>**  
# 目录  
- [一、总则](#一总则)
  - [1.名称与目的](1-名称与目的)
  - [2.开源协议与去中心化](2-开源协议与去中心化)
  - [3.代码审计与隐私数据](3-代码审计与隐私数据)
- [二、共识机制](#二共识机制)
  - [1.节点类型](1-节点类型)
  - [2.交易验证](2-交易验证)
  - [3.超级节点](3-超级节点)
  - [4.权威节点](4-权威节点)
  - [5.权益节点](5-权益节点)
  - [6.全节点](6-全节点)
  - [7.轻节点](7-轻节点) 
- [三、发行与销毁](#三发行与销毁)
  - [1.创世发行](1-创世发行)
  - [2.省储行创立发行](2-省储行创立发行)
  - [3.创世发行](3-全节点发行)
  - [4.创世发行](4-轻节点发行)
  - [5.创世发行](5-质押发行)
  - [6.创世发行](6-决议发行) 
- [四、投票机制](#四投票机制)
  - [1.拒绝发行或销毁](1-拒绝发行或销毁)
  - [2.增加/减少省储会、省储行节点](2-增加/减少省储会、省储行节点)
  - [3.拒绝增加/减少省储会、省储行](3-拒绝增加/减少省储会、省储行)
- [五、费用及利息](#五费用及利息)
  - [1.交易费](1-交易费)
  - [2.借贷利息](2-借贷利息)
  - [3.质押利息](3-质押利息)
- [六、创世理念](#六创世理念)
  - [1.《五民宪章》第一条](1-《五民宪章》第一条)
  - [2.《五民党章》第一条](2-《五民党章》第一条)
  - [3.我们不反共，我们只灭共！](3-我们不反共，我们只灭共！)
  - [4.我们的口号](4-我们的口号)
****
# 一、总则
## 1. 名称与目的  
* 基于区块链技术用于推动由五民党发起的<<去中心化的民主运动>>的新型法定货币系统———<<五民币金融区块链系统>>；
* 五民币金融区块链系统是一个不受任何机构掌控的新型法定货币系统，一个所有人都能自由使用的新型法定货币系统，一个所有中华民族联邦共和国公民都能参与网络治理投票的新型法定货币系统；
* 创立五民币金融区块链系统的目的是推广去中心化的民主运动，早日推翻共产党的邪恶统治，以建立自由民主的中华民族联邦共和国。  
## 2. 开源协议与去中心化
* 五民币金融区块链系统及其附属软件采用BSD开源协议，并在GitHub等多个开源平台上完全开放源代码；
* 去中心化是保障五民币金融区块链系统安全最重要的方式，本系统完全去中心化，任何个人、任何组织都可以加入本系统成为去中心化节点（全节点、轻节点）之一。  
## 3. 代码审计与隐私数据
* 本系统开源代码的审计工作由<<五民币金融区块链系统开源代码审计委员会>>和其他任意第三方机构共同审计，审计委员会定期提交代码审计报告，任何第三方可随时审查本系统源代码；
* 在五民币金融区块链系统上，用户可以绝对掌握自己的隐私、数据和资金，没有中心化的服务器偷窥用户隐私、窃取用户数据，没有他人能侵占用户资金，完全由用户掌握，用户可以随时删除自己的数据，转移自己的资金。  
****
# 二、共识机制
## 1. 节点类型
### A、超级节点  
* 超级节点负责生成国储会、省储会、省储行初始节点和生成第1个至第43个区块，超级节点分为初始超级节点状态、正常超级节点状态和降权超级节点状态共三种状态。  
### B、权威节点  
* 初始权威节点共有44个，其中，国家储备委员会为1个初始权威节点、初始43个省储备委员会每个省储会为1个初始权威节点，初始权威节点不能被删除；
* 后新增省储会的，每新增1个省储会即增加1个权威节点，新增的权威节点可被删除。  
### C、权益节点  
* 初始权益节点共有43个，初始43个省储备银行每个省储行为1个初始权益节点，初始权益节点不能被删除；
* 后新增省储行的，每新增1个省储行即增加1个权益节点，新增的权益节点可被删除，省储行权益节点为验证者节点。  
### D、全节点  
* 全节点采用PoW共识，负责全链所有新区块（新区块特指除创世区块和第1个至第43个区块以外的所有区块）的铸造，独享交易费和铸块发行的数字五民币。  
### E、轻节点  
* 轻节点主要用于移动支付和网络治理投票，经公民认证的轻节点（WuminApp）拥有投票权，无需认证的访客轻节点（WuminBi）没有投票权。  
### F、节点ID与节点软件  
* 除无需认证的WuminBi轻节点和全节点外，其他所有节点都拥有唯一的节点ID用以标识节点身份，且每种类型的节点都拥有专用的节点软件。  
## 2. 交易验证
* 交易采用PoS权益验证，验证者节点由所有权益节点组成，即各省储行节点。
* 有超过90%的验证者节点验证交易合法性后即可交由全节点铸造新块，全节点采用PoW工作量证明机制获得铸块权。  
## 3. 超级节点
* 初始状态的超级节点拥有管理权威节点和权益节点的权限，可生成初始权威节点和初始权益节点，并生成第1个~第43个区块，及生成、删除权威节点和权益节点管理员的权限；
* 初始状态的超级节点在完成44个权威节点和43个权益节点的生成后，即时转换为降权超级节点状态，降权状态的超级节点仅拥有钱包收入和支出两项权限；
* 由国储会权威节点发起，省储会权威节点和权益节点授权同意，且超过77%的可投票轻节点授权同意的，可恢复超级节点的降权状态为正常状态；
* 国储会发起恢复超级节点为正常状态的，必须列明理由和时限，且恢复为正常状态的超级节点的所有操作，必须经权威节点和权益节点的授权同意方可生效；
* 对于恢复正常状态的超级节点的所有操作，有超过77%的可投票轻节点投票否决的，可否定正常状态的超级节点的所有操作；
* 超过正常超级节点状态时限的，立即转变正常超级节点状态为降权超级节点状态；
* 超级节点使用超级节点软件，且只拥有一个钱包地址，超级节点拥有5个管理员，其所有操作至少需要有3个管理员授权；
* 正常超级节点状态的超级节点拥有生成和删除权威节点、权益节点管理员的权限，拥有增加/删除新权威节点和权益节点的权限。  
## 4. 权威节点
### A、国储会  
* 国储会权威节点拥有1个多签名地址，该多签名地址拥有19个管理员；
* 国储会权威节点19个管理员有13个及以上管理员签名同意的，即可生效且19票同时生效；
* 国储会权威节点使用国储会权威节点软件，且国储会权威节点没有钱包地址。
### B、省储会  
* 每个省储会权威节点拥有1个多签名地址，该多签名地址拥有9个管理员；
* 省储会权威节点9个管理员有6个及以上管理员签名同意的，即可生效；
* 省储会权威节点使用省储会权威节点软件，省储会权威节点拥有1个钱包地址。
### C、共同规则  
* 国储会权威节点拥有19票投票权，省储会权威节点每1个节点拥有1票投票权；
* 对于权威节点的投票，有超过65%的权威节点投同意票的即通过，反之则否决；
* 国储会权威节点软件能发起发行和销毁数字五民币、增加/减少新省储会和新省储行、改变超级节点状态等流程；省储会权威节点能发起发行和销毁数字五民币等流程；
* 省储会单独销毁本省储会持有的数字五民币的，仅需本省储会通过即可,且无需轻节点投票。  
## 5. 权益节点  
* 每个权益节点拥有一个多签名节点地址和两个钱包地址，使用省储行权益节点软件；
* 每个省储行权益节点的多签名地址有19个管理员，对节点的所有操作需超过13个管理员的授权；
* 每个省储行权益节点拥有1个质押钱包地址，该钱包仅用于质押省储行创立发行的数字五民币,该钱包不能进行任何操作；
* 每个省储行权益节点拥有1个储蓄钱包地址，该钱包地址用于省储行日常业务及接收质押利息等；
* 对于权益节点的投票，有超过65%的权益节点投同意票的即通过，反之则否决。  
## 6. 全节点  
* 全节点数量不限，部署运行全节点软件的即是全节点，全节点负责系统所有新区块的铸造；
* 任何组织、任何人均可下载安装全节点软件,成为全节点，且全节点承担归档节点的功能，储存区块链的所有数据；
* 全节点独享全网所有交易手续费，采用PoW工作量证明共识获得铸块权。  
## 7. 轻节点  
### A、公民轻节点
* 公民类型的轻节点集成于WuminApp内，需要注册WuminApp并使用联邦公民身份识别码认证，拥有投票权；
* 公民类型的轻节点，其轻节点ID、钱包地址、WuminAppID和公民身份识别码四要素一对一绑定，完成绑定后方可参与投票；
* 公民类型的轻节点只能拥有1个钱包地址，注销轻节点ID的，同时删除钱包地址，且解除四要素绑定关系；
* 公民类型的轻节点超过10年时间没有活跃的，系统将删除该节点以释放资源，系统删除节点的同时解除四要素绑定关系。  
### B、访客轻节点
* 访客类型的轻节点无需注册，下载安装WuminBi钱包App即可使用，没有投票权；
* 访客类型的轻节点可生成N个钱包地址；
* 访客类型的轻节点超过10年时间没有活跃的，系统将删除该节点以释放资源；
* 使用WuminBi钱包App生成的钱包地址，在超过10年时间余额持续低于1元五民币的，系统将删除该钱包地址以释放资源。  
****
# 三、发行与销毁
## 1. 创世发行
* 创世发行1,443,497,378元数字五民币，由创世区块发行，创始者所有；  
* 创世发行量以中共第7次人口普查数据为基准，即每一个人发行1元；  
* 创世发行寓意每一个中国人支付1元，以支持五民主义和去中心化的民主运动。  
## 2. 省储行创立发行
* 从第1个区块至第43个区块，每个区块生成一个省储行节点，并发行该省总人口数x100的数字五民币；  
* 各省储行实际发行量以中共第7次人口普查数据为基准，共计发行144,349,737,800元；  
* 各省储行发行量之和可少于144,349,737,800元，但不得超过144,349,737,800元；  
* 省储行创立发行的数字五民币质押于该省储行质押钱包地址，质押资金永久锁定，质押利息由质押发行支付；  
* 省储行创立发行的数字五民币质押时间满100年后，系统永久停止质押发行，永久停止支付质押利息；  
* 省储行创立发行并永久质押的数字五民币是证明该省储行合法性和权益来源的依据。  
## 3. 全节点发行
* 运行全节点的每铸造一个新区块，系统发行99999元数字五民币用于奖励该节点；  
* 全节点铸块发行为第44个-第99999个区块，共计发行量为9,995,500,044元数字五民币；  
* 当本区块链系统的区块高度超过99999个区块后，系统即永久停止全节点发行。  
## 4. 轻节点发行
* 使用中华民族联邦共和国公民身份识别码完成轻节点认证后，将获得认证奖励，该奖励在轻节点认证完成后系统自动发行并汇入该轻节点钱包地址；  
* 轻节点认证发行总量累计未超过144,349,737,800元时，每个轻节点认证发行9999元数字五民币；轻节点认证发行总量超过144,349,737,800元后，每个轻节点认证发行999元数字五民币；  
* 轻节点总计发行量为：1,429,060,961人*999元=1,427,631,900,039元+144,349,737,800元=1,571,981,637,839元数字五民币；  
* 获得9999元发行奖励的轻节点，所获得的发行奖励无需锁定；获得999元发行奖励的轻节点，所获得的发行奖励处于锁定状态，在持有该轻节点的公民完成《公民素质教育》题库学习后，凭结业证书即可解锁奖励资金；  
* 达到轻节点发行总量后，再认证的轻节点不再新发行不再获得奖励，认证发行奖励以先完成认证优先获得，每个公民身份识别码仅能认证一次；  
* 轻节点发行的主要目的是要让更多的公民参与去中心化的民主运动，早日推翻共产党的邪恶统治，让更多的公民加入网络治理，以提高系统安全性。  
## 5. 质押发行
* 各省储行质押的数字五民币，由系统质押发行支付100年的质押利息,年利率固定为5‰,由系统以智能合约方式在每个自然年结束后自动发行并存入各个省储行储蓄钱包地址；  
* 质押发行是为了支付省储行质押本金所产生的利息，质押发行的数字五民币归各省储行所有；  
* 在各省储行创立发行的数字五民币质押满100年后，即永久停止质押发行，不再支付质押利息。  
## 6. 决议发行
* 成立公民储备委员会后，由联储会决议发行或销毁数字五民币，经联储会联合会议决议通过后，由权威节点发行和销毁；  
* 适时发行纸质五民币，纸质五民币用以替换人民币，数字五民币与纸质五民币按1:1兑换，五民币与人民币按自由汇率兑换；  
* 纸质五民币面额由1分、5分、10分、50分、1元、5元、10元、20元、50元、100元共10种面额组成；  
* 新发行的五民币由各省储会按联储会决议分配给各省储行，省储行再将其借贷给辖区商业银行；销毁五民币的按联储会决议销毁各省储行持有的五民币。  
* ****
# 四、投票机制
## 1. 拒绝发行或销毁
* 非全票通过的联储会决议，有超过全链65%的可投票轻节点投票否决发行或销毁数字五民币决议的，可否定联储会发行或销毁数字五民币的决议；  
* 满足联储会全票通过，且所有省储会和省储行节点均全票通过的决议，可直接生效，不再进入轻节点投票环节。  
## 2. 增加/减少省储会、省储行节点
* 在超级节点处于降权状态时，由国储会权威节点发起增加/减少新省储会、新省储行节点的投票，有超过65%的权威节点投票同意的，即可增加/减少新省储会、新省储行节点；  
* 增加/减少新省储会、新省储行的，必须先有国家立法院表决通过后的增加/减少省级行政区的决议为依据，国储会权威节点才能发起投票流程。  
## 3. 拒绝增加/减少省储会、省储行
* 超过全链65%的可投票轻节点投票否决增加/减少新省储会、新省储行投票的，可否定权威节点的投票结果。  
****
# 五、费用及利息
## 1. 交易费
* 数字五民币的所有链上交易均收取1‰的交易手续费，交易手续费全部用于奖励铸块全节点；  
* 纸质五民币的交易手续费不适用于此条共识规则。  
## 2. 借贷利息
* 储委会新发行的五民币，由发行决议分配给各省储行持有，各省储行将持有的五民币借贷给辖区商业银行以获得贷款利息收益；
* 各省储行持有的五民币，由该省省储会决议使用规则，该省省储行有义务按决议规则执行。  
## 3. 质押利息
* 各省储行质押的省储行创立发行的数字五民币，系统给予每年5‰，累计支付100年的质押利息，满100年后即永久停止支付质押利息；
* 质押利息由质押发行支付，质押利息属于各省储行权益节点的收益。  
****
# 六、创世理念
## 1.《五民宪章》第一条
* 先有人类后有国家，是公民建立国家，国家是公民的国家，是公民治理国家，而不是国家统治公民，公民没有爱国的义务；国家政权的建立其基本原则是保护公民的生命权、自由权、财产权、反抗压迫权和选举与被选举权不受任何的非法侵犯，当国家政权无法保证这一基本原则时，公民有权有义务推翻这个政权，建立一个以保障公民生命权、自由权、财产权、反抗压迫权和选举与被选举权为基本原则的政权。  
## 2.《五民党章》第一条
* 本党立党之理念：中华民族联邦共和国民治民主民权民生民族党，简称中国五民党、五民党或本党；中国五民党是自由的、民主的、正义的、平等的、求实的政党，本党基于自由之思想、民主之制度、正义之精神、平等之人权和求实之言行的“五之理念”立党；为保障立党之理念的推广、保障立党之目标的实现，特制定中国五民党党章，简称五民党党章、本党章或党章。  
## 3.我们不反共，我们只灭共！
* 确定一个理念、设定一个目标、制定一套规则，聚沙成塔、推翻中共！我们不反共，我们只灭共！中国五民党。  
## 4.我们的口号
* 驱逐马列邪教，恢复中华正统；解散共产匪党，建立中华联邦；抵制独裁专制，执行民主宪政！驱逐马列，恢复中华；解散共党，建立联邦；抵制独裁，执行宪政！  
