**SOLANA ON-CHAIN INVESTIGATION & FORENSICS** 

# **Transaction Detective: Анализ реальных транзакций в сети Solana** 

Работу выполнил: Рамазанов А.Д. DS 

Сравнительный ончейн-анализ: DEX Swap (Raydium CLMM) vs. MEV Arbitrage (Jupiter & Meteora) Дата формирования отчета: Сентябрь 2026 

**Цель исследования:** Детальный технический разбор двух архитектурно противоположных транзакций в экосистеме Solana. Рассматриваются механизмы авторизации (Durable Nonce vs ALT), глубина межпрограммных вызовов (CPI), распределение вычислительных лимитов (Compute Units) и экономическая модель MEV-арбитража. 

## **Транзакция 1: Своп на концентрированной ликвидности Raydium (DEX Swap)** 

##### **Solscan URL:** 

https://solscan.io/tx/pa3bjQBfYUnLMkT4QnkGMBBNyJCfjiqJov1uoCUa4oJtFudUd9ibLsejbWzJvjSRbasWcCJKsgD64YWUPpZrxnG 

### **1. Общие метаданные** 

|**Signature**|pa3bjQBfYUnLMkT4QnkGMBBNyJCfjiqJov1uoCUa4oJtFudUd9ibLsejbWzJvjSRbasWcCJKsgD64YWUPpZrxnG|
|---|---|
|**Slot / Block**|450635074|
|**Статус**|Success / Finalized (MAX confirmations)|
|**Версия транзакции**|Legacy (прямой список всех аккаунтов в заголовке)|
|**Комиссия (Fee)**|0.000006485 SOL (5 000 Lamports Base + 1 485 Lamports Priority Fee)|
|**Compute Units (CU)**|39 236 использовано из лимита 66 549 (эффективность: 58.96%)|
|**Суть операции**|Обмен 535.0267982 токенов 2Z (~$41.14) на 41.892861 USDC (~$41.89) через пул Raydium CLMM.|



### **2. Задействованные программы (Programs)** 

**• System Program** (11111111111111111111111111111111): Нативная программа Solana. Использована для продвижения durable nonce (advanceNonce), что позволяет подписать транзакцию офлайн без риска инвалидации по недавнему blockhash. 

- **Compute Budget Program** (ComputeBudget111111111111111111111111111111): Системная программа для аллокации вычислительных ресурсов валидатора и установки приоритетной платы за транзакцию. 

**• Custom Routing / Bot Contract** (9VFiimgx8hReh6TqamMFXFRXrtsZUfFGuoBTcW3jb7n8): Пользовательская программапрослойка (потребовала всего 250 CU). Служит триггером или верификатором условий перед исполнением основного ордера. 

**• Raydium Concentrated Liquidity** (CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaWDAwpqcQFqk): Программа пулов концентрированной ликвидности (CLMM). Рассчитывает тики цены, изменение резервов и размер проскальзывания (slippage). 

- **SPL Token Program** (TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA): Базовая программа экосистемы Solana, управляющая состоянием токен-счетов и физическим перемещением токенов. 

### **3. Классификация аккаунтов (Accounts)** 

|**Роль аккаунта**|**Публичный адрес / Идентификатор**|**Назначение в транзакции**|
|---|---|---|
|**Signer, Writable (Fee Payer)**|9nXDunV8eNvYSVys7JSBkm2hz9Q79Q3EYeko19srVeMC|Кошелёк пользователя. Подписал транзакцию,<br>оплатил комиссию в SOL, владеет токенами.|
|**Writable**|ATA пользователя под токен 2Z|Associated Token Account; баланс уменьшился<br>на 535.0267982 2Z.|
|**Writable**|ATA пользователя под токен USDC|Associated Token Account; баланс увеличился на<br>41.892861 USDC.|
|**Writable (PDA)**|Raydium (USDC-2Z) Vaults & Pool State|Программно-управляемые аккаунты (PDA)<br>хранилищ пула, в которых пересчитываются<br>резервы пула и тики ликвидности.|
|**Read-Only (Executable)**|System Program, Compute Budget, Raydium CLMM, SPL<br>Token|Программы (код), логика которых вызывалась<br>для исполнения логики без изменения их<br>собственного стейта.|



### **4. Дерево инструкций (Instructions & CPIs)** 

**Instruction #1 — System Program (advanceNonce):** Продвигает Nonce-аккаунт для защиты от replay-атак без привязки к скользящему 150-блочному окну RecentBlockhash. 

**Instruction #2 — Compute Budget (SetComputeUnitLimit):** Выделяет лимит в 66 549 CU. 

**Instruction #3 — Compute Budget (SetComputeUnitPrice):** Задаёт приоритетную комиссию в micro-lamports на единицу CU для ускорения валидаторами. 

**Instruction #4 — Unknown Program (9VFiim...):** Сервисный вызов смарт-контракта (потребление 250 CU). 

**Instruction #5 — Raydium Concentrated Liquidity (swap)** 

Основная бизнес-логика (38 536 CU): 

- ↳ **Inner Instruction 5.1 (CPI → SPL Token Program):** Вызов Transfer — списание 535.0267982 2Z с токен-аккаунта 

- пользователя в хранилище пула. 

↳ **Inner Instruction 5.2 (CPI → SPL Token Program):** Вызов Transfer — перевод 41.892861 USDC из хранилища пула на токенаккаунт пользователя под контролем PDA-подписи программы Raydium. 

## **Транзакция 2: Арбитражный своп через маршрутизатор Jupiter v6 (MEV / DEX Arbitrage)** 

##### **Solscan URL:** 

https://solscan.io/tx/2LsFoxXfPtK6wJMrvQsn6PhKMk2ge6HguwoZAN3e9ZCy2ywXwcri1rQnyrs8HVdNyyqdrAAUzfdcd43DY8bdZjZ 

### **1. Общие метаданные** 

|**Signature**|2LsFoxXfPtK6wJMrvQsn6PhKMk2ge6HguwoZAN3e9ZCy2ywXwcri1rQnyrs8HVdNyyqdrAAUzfdcd43DY8bdZjZ|
|---|---|
|**Slot / Block**|450486902|
|**Статус**|Success / Finalized|
|**Версия транзакции**|Version 0 (v0) — использует Address Lookup Tables (ALT)|
|**Address Lookup Tables**|B43NUWjh... и edvM3tz1...|
|**Комиссия (Fee)**|0.00006559 SOL (высокая приоритетная комиссия 0.00006059 SOL — признак арбитражного бота)|
|**Compute Units (CU)**|111 039 использовано из лимита 132 520 (83.79%)|
|**Суть операции**|Атомарный циклический арбитраж (Swap Arbitrage). Бот отдал 0.061485394 WSOL и получил<br>0.06157118 WSOL, зафиксировав чистую прибыль +0.000085786 WSOL за счет спреда цен между<br>пулами Raydium v4 и Meteora.|



### **2. Задействованные программы (Programs)** 

- **Compute Budget Program** (ComputeBudget111111111111111111111111111111): Задаёт лимиты CU, цену приоритета и максимальный размер загружаемых данных аккаунтов (setLoadedAccountsDataSizeLimit). 

- **Custom Bot Proxy Contract** (NinafKYvKDCH26v6uEpfDjyuDjjpbdfhiPjrJV6FTFs): Смарт-контракт арбитражера, оркеструющий транзакцию и проверяющий условие прибыльности (если профита нет — транзакция откатывается / revert). 

- **Jupiter Aggregator v6** (JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4): Маршрутизатор ликвидности, строящий мультипуловый маршрут обмена. 

- **Raydium Liquidity Pool V4** (675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8): Классический AMM-пул Raydium (Constant Product k = x * y). 

- **Meteora Pools Program** (LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo / Dynamic Pool): Программа пулов ликвидности экосистемы Meteora. 

- **Meteora Vault Program** (24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi): Хранилище ликвидности с механизмами динамического начисления доходности. 

- **SPL Token Program** (TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA): Базовое исполнение переводов обмениваемых токенов. 

### **3. Классификация аккаунтов (Accounts) и особенности Versioned Transactions** 

**Архитектурная заметка:** Использование транзакции Version 0 (v0) позволило упаковать более 20–30 аккаунтов через таблицы поиска адресов (ALT). В Legacy-транзакции это гарантированно вызвало бы ошибку превышения лимита IPv6 MTU (1232 байта). 

|**Роль аккаунта**|**Публичный адрес / Тип**|**Описание роли**|
|---|---|---|
|**Signer, Writable (Fee Payer)**|GxDC9e7SP9mzhDo4re5HbpLa2RW7gB9DtmThx4i4pXSq|Кошелек арбитражного бота, оплативший<br>высокую комиссию ради первоочередного<br>включения в блок.|
|**Writable**|Временный токен-аккаунт бота под WSOL|Источник стартового объема и конечный<br>получатель зафиксированного профита.|
|**Writable (PDAs)**|Raydium V4 Vaults & Reserves|Аккаунты пула Raydium, где резерв<br>промежуточного токена изменился в первом<br>шаге обмена.|
|**Writable (PDAs)**|Meteora Pool & Vault Accounts|Аккаунты пула и хранилища во втором шаге<br>арбитражного цикла.|
|**Read-Only (Lookup Tables)**|B43NUW..., edvM3t...|Таблицы адресов (ALT), хранящие 32-байтные<br>публичные ключи для компактной адресации<br>по 1-байтному индексу.|
|**Read-Only (Executable)**|Jupiter, Raydium, Meteora, Token Program|Исполняемые программы блокчейна.|



### **4. Дерево инструкций и механизм Cross-Program Invocation (CPI)** 

**Instruction #1–#3: Служебная настройка через Compute Budget:** SetComputeUnitLimit (132 520 CU), SetComputeUnitPrice (высокий приоритет для валидаторов), setLoadedAccountsDataSizeLimit (оптимизация аллокации оперативной памяти валидатора). 

**Instruction #4 — Вход в контракт бота (NinafKYv...):** Главная арбитражная оркестрация со вложенным стеком CPI: 

- ↳ **CPI #1 → Jupiter Aggregator v6 (Route):** Маршрутизация свопа через агрегатор ликвидности. 

   - ↳ **CPI #2 → Raydium Liquidity Pool V4:** Выполняет первую ногу обмена: WSOL → промежуточный токен. 

      - ↳ **CPI #3 → SPL Token Program:** Перевод WSOL со счёта бота в пул Raydium. 

      - ↳ **CPI #4 → SPL Token Program:** Выдача промежуточного токена боту. 

- ↳ **CPI #5 → Meteora Pools Program (Swap):** Вторая нога обмена: промежуточный токен → обратно в WSOL. 

      - ↳ **CPI #6 → SPL Token Program:** Трансфер промежуточного токена в пул Meteora. 

   - ↳ **CPI #7 → Meteora Vault Program (Deposit):** Балансировка хранилища Meteora (включает 2 внутренних перевода через 

   - Token Program). 

   - ↳ **CPI #8 → Meteora Vault Program (Withdraw):** Вывод целевых WSOL токенов из хранилища (ещё 2 внутренних перевода 

   - через Token Program). 

**Завершение маршрутизации Jupiter:** Возвращает в ончейн-лог байты успешного исполнения: Return: JUP6LkbZbj... bICrAwAAAAA=. 

**Проверка прибыльности ботом:** Контракт бота фиксирует, что баланс WSOL на выходе превысил баланс на входе (профит зафиксирован), и завершает вызов (Return: NinafKYv...). 

**Instruction #5 — Завершающий вызов контракта бота (NinafKYv...):** Проверка инвариантов и очистка временного состояния (потребовала всего 12 CU). 

## **Итоговое сравнение для отчёта** 

|**Критерий**|**Транзакция 1 (Raydium CLMM)**|**Транзакция 2 (Jupiter / Meteora)**|
|---|---|---|
|**Формат транзакции**|Legacy (прямой список всех адресов в<br>заголовке)|Version 0 (v0) с Address Lookup Tables (ALT)|
|**Инициатор**|Пользователь / Офлайн подпись (Durable<br>Nonce)|MEV / Высокочастотный арбитражный бот|
|**Глубина стека CPI**|2 уровня (Raydium → Token Program)|4 уровня (Bot → Jupiter → Meteora → Vault<br>→ Token Program)|
|**Характер вычислений**|Линейный обмен в рамках одного пула|Многопрограммная атомарная композиция<br>(Flash-swap / Arbitrage)|
|**Потребление CU**|39 236 CU (58.96% от лимита)|111 039 CU (83.79% от лимита)|
|**Структура комиссии**|Минимальная (0.000006485 SOL,<br>стандартный приоритет)|Агрессивная (0.00006559 SOL, x10 за счет<br>Priority Fee)|
|**Защитные механизмы**|Durable Nonce против replay и устаревания<br>blockhash|Атомарный revert в контракте бота при<br>отсутствии арбитражного спреда|



#### **Ключевой технический вывод:** 

Представленный анализ наглядно демонстрирует эволюцию исполнения инструкций в Solana. Если первая транзакция отражает классический пользовательский сценарий с рутинным линейным исполнением и использованием механизма Durable Nonce, то вторая представляет собой вершину composability блокчейна — многоуровневую атомарную арбитражную операцию, использующую сжатие через Address Lookup Tables (ALT), глубокую вложенность CPI и агрессивную приоритизацию газа для гарантированного включения в слот валидатором. 

