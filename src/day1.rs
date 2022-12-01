fn main() {
    let input = _get_input();

    // ----------- Parse Input -----------

    let parsed = input
        .trim()
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .map(|s| s.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    // ----------- Solve -----------

    let mut sums = parsed
        .iter()
        .map(|v| v.iter().sum::<i32>())
        .collect::<Vec<i32>>();
    sums.sort_by(|a, b| b.cmp(a));

    // ----------- Print -----------

    println!("Part 1: {}", sums[0]);
    println!("Part 2: {}", sums[0] + sums[1] + sums[2]);
}

fn _get_test_input() -> String {
    return "

1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

"
    .to_string();
}

fn _get_input() -> String {
    return "

7914
5032
11424
2567
4123
3567
7346

1334
2173
5437
1104
1872
1148
6547
3149
5923
5705
4036
5348
1100

4108
14444
25596

7237
4252
1020
8333
3538
5089
6328
4581
6503
8741

5799
1585
1137
1985
6489
5309
6249
4594
1885
6100
4233
1517
6346

13392

5705
2914
1636
2126
1200
2458
3862
5696
6269
6624
4015
6878

7503
4528
1771
4976
2360
4953
8402
5437
1521
3369

19947
3465

2635
3520
5764
1201
2231
5663
1872
1834
5889
3104
6043
5672
1897
4429
3242

1568
7727
7363
1762
6400
5605

4660
11308
4380
2582
1001
11714

5134
2300
4757
4613
2991
6561
5951
4936
4592
2099
4117
6658
6387

24950
7865
7757

2251
6418
1692
5020
5913
1070
3996
6016
2406
6237
4374
1631
6892

1679
4298
3371
3474
4722
4311
1610
3863
6770
4010
5611
2097

6922
5309
7222
5672
6050
1269
4328
3736
4861
3956
3206
1832

1880
3478
7714
7708
6067
6378
3506
7447
5526
6263
6018

6731
5938
6918
1352
1828
5339
4113
5783
2637
3584
1874
2647

2781
6009
6301
6635
4924
4150
2216
6989
3632
5509
4650

20208

1060
9513
1929
6742
8104
7913
5924
4176
3578

5296
9369
2530
7070
8392
9054
3721
4291
9210

3334
2939
3462
1612
3295
1080
1286
4522
1815
2245
4000
5780
5722
4194
4371

3400
4587
1020
8609
7675
1272

1716
6587
4563
3922
3788
6667
3734
6692
1783
3153
4195

2572
6830
3179
1029
2269
11843
9941

6543
6760
15219
11241
10592

6608
1237
3951
3068
3073
3796
3603
3223
3937
5450
5436
3069
1061

9396
11859
1902
6598

3137
1487
2503
1312
3008
1914
4730
6104
2293
2913
2729
3629
5186
3876
3404

9051
6493

8009
5306
4188
6706
7077
2134
7837
2885
6423
6267
1222

4293
4554
5378
6061
3238
3792
2238
5014
3508
2296
5479
5254
1030
1644

15277
4285
10110
7055

4761
4290
1238
1174
2299
1306
2659
4854
5089
4261
3890
1556
5211
4021
3356

10403
21723

5893
4236
5927
2052
3192
1468
4598
3233
2168
3106
4510
1847
5083
2421

16205
15366
21046

6114
12370
10780
9663
9061

4516
2854
1460
2143
6609
2365
2778
2683
2583
6467
4319
4315

3799
6044
2288
6494
3439
7232
3414
1388
3966
2398

4225
4485
3923
4014
1199
1468
2776
2777
2377
1434
4292
1456
3289
5520
4329

4510
3831
3755
3459
3391
2348
6074
3206
3029
2843
3713

5998
3721
3166
4632
5031
2793
1167
4946
1248
5716
5348
1525
3757
4700
3372

3952
7922
3443
4604
6810
2816
6138
2392
1241
4236
3619

34427

17838
30517

4112
10150
8347
2082
12014
7078
11182

5433
7853
1534
11283
5337

6007
4147
2008
4531
5174
5625
5105
1196
1521
2353
6811
1187
2310

1886
2979
6398
6356
7702
8755
11455

5357
1608
5575
6241
4128
3130
6163
3212
4485
5898
4910
3788

7801
7457
7253
8529
9551
5943
1356
6505

4727
8200
6628
6451
1375
1678
5936
5324
6427
2029

36582

7271
35840

21992
22165

6453
19268
3430
10457

4029
4655
1781
7635
5762
2584
3564
2071
3439
3691
3094

9444
13140
13396
4076
10162
5268

2126
4467
9082
8317
6550

5451
6635
6791
5015
8407
4356
2602
3230
4665

2747
7765
6817
4816
3437
7966
5722
2444
3083
1361

12228
6182
7426
12898
4182
6266

6539
6363
4724
5591
4163
2589
1828
1351
6077
1146
3987
6487
6049

6827
2987
3205
4405
1129
6673
4913
2839
2060
2537
2022
1432

3777
3777
4089
3379
2110
2320
1109
3404
6876
2791
4787
5419

12034
17906
5562
5606

14336
4651
2209
11190
6228

4697
4733
1356
2000
5179
2664
2768
5424
2590
1267
3390
2164
4090
3494
5899

2192
13670
13944
4531
3025
11249

5830
14261

19801
4961
12629
9004

7113
7210
3654
6763
7717
4323
5696
5946
2384
7844
1486

7010
5874
1878
6510
1164
6037
5962
1137
2150
2233

14434
4226

3411
13559
4848
4821

15749
2913

54246

11361
2720
3767
5540
3472
4367
6511

6896
4369
2970
2445
2070
2562
5328
3920
5450
2897

14667
15686
1271
18275

6705
2670
11190
13652
9738

4020
4975
7099
4654
8754
5562
7225
3903
4546
4525

4197
14210
7595
10871
7217

4815
5253
7706
2919
4097
2700
2721
3093
5273
1696
5776

2771
4272
3324
3439
6376
5354
5007
6146
4939
1351
6437
5618
4283

8893
5075
4562
9003
8963
7147
2381

2911
11753
12512
3446
6006
11130

2478
1060
6245
1266
5077
2586

4400
5838
6425
2635
3060
1709
6254
2813
2120
1735
4942
5332
3626

5600
3354
4776
2979
2700
4637
1717
2134
1782
3492
3611
4941
4899
1213

8679
4661
10003
13830
5247
2139

20373
3325

5379
7303
7830
5504
2696
5622
7935
3297

4228
2408
5941
5082
6872
5948
6254
3644
3301
5938

5258
23981
12886

28519
27915

35254
2202

7052
2168
4389
4046
2056
6167
3384
6470
7987
3383

5268
7903
13452
14317

6366
1526
9104
2599
1202
11086
5433

11479
8039
10152
14580

5515
1916
1289
1388
1553
5740
3839
1753
5634
6645
1506
4538
4656

4786
4267
4796
2879
3770
5652
3160
5825
2297
1351
3115
5746
4752
4713
5409

5911
6004
4125
3494
7580
7176
8214
6670

2668
7245
3101
4052
1527
6544
3423
5764
2754
1405
3936

5601
3717
1629
5576
5579
1855
4257
5682
2532
2718
1497
3427
5794

5247
11031
4614
9068
9698
2157
5376

9323
2194
1541
5612
1785
1866
8066
4904

36692
27998

4516
3456
3620
10427
7221
9173
5353
9376

6685
3452
9421
7877
7669
3237
9391
5727

20001

10724
15712
15690
16206
1635

26550
19421

12539
8286
13842
2148
4421

10165
10293
5777
8034
8797
3886

5877
3703
1191
5264
1433
4417
2501
6113
1390
1953
1989
4148
4308

5421
2719
2110
2329
2480
5456
2894
1536
3655
2022
4919
2287
1146
3599
2766

23545
23885

3539
18515

31310

3802
13051
2107
11803
11383
10824

7070
3331
1545
1227
3701
4290
2178
4043
4159
4978

1596
5306
5192
7398
7306
7457
2157
4592
7133
5658
6134
5000

11745
7176
6232
13572
3855

6991
1973
6491
6894
6839
3710
6206
3259
7334
3714
4320
4423

1602
18051
13165
19153

1414
13321
3488
6907
1765
11847

6007
13509
10855
2103
11112
1317

9168
10364
8969
7162
1829
1114
1280
8882

8113
3195
8998
1596
3238
5872
8194
1753

5109
2221
3304
5088
3968
5134
1064
5486
5735
3521
4646
5617
2802
5293

5915
6651
6734
6670
1891
6987
3404
1946
9510

11531
3497
11740
11651
1885
7156
9233

6541
7739
1454
6086
2743
6713
5321
6442
6950
3678
3234

6800
5686
8043
9452

19119
19636
2349
2244

5008
7002
5330
3546
13001
13785

4985
7177
5629
4174
6747
5113
3916

6496
5247
6753
6704
9306
5765
4150
6939
5457

3887
6986
1560
8183
4405
3810
5728
10507

11447
25816

4597
5623
4127
5048
8158
2593
2250
5352
1893
6507

33979

6089
5585
2486
5620
1347
5829
4476
5265
2216
4211
1707
2443
2362
2362
1282

5883
2431
5837
2697
6104
5978
5515
5215
5534
2081
1031
2757
1034
5281
5333

3373
1608
3753
1177
3197
1753
2510
5515
5455
4239
6135
5620
5610
2864

3836
6195
5633
2714
1886
6240
1274
1432
1305
2310
3446
6485

8674
19094
22441

6453
2960
1048
1714
6828
6116
4645
3327
4146
2561
2149
2429
1382

27003
2645

3984

4217
2174
6227
6827
2477
2789
3961
3937
4771
3640
1870
5929
1949

5251
4132
2290
5517
5219
3745
3219
4119
3056
1951
6167
5802
5301
6245

6430
5525
6656
5755
9884
6570
1134

7728
4025
7558
4767
1345
2164
1137
1174
1130
5195
6515

4223
3412
9634
5355
5395
5975
2627

6170

10254
7145
6816
9867
2471
1584
8533

3486
9633
10625
6684
3246
9866
10563
1430

5318
3542
4517
3296
3776
4122
4940
2768
4821
3779
1616
5543
3953

14860
10176
13551
6410
6235

5075
2852
3919
3024
5265
2946
4377
5901
5427
5960
2953
5508
6309
2532

4346
1003
2014
1816
7083
7229
6297
6349
3860
1911
4553

10672
2514
4572
6904
4101
7105
3486
5939

4903
2768
6198
3331
5677
4852
1123
1105
5599
7130
6093

36680

2386
4584
1424
4818
2181
1275
1203
6859
1722
3839
1602
7022

7947
11689
6053
2557
11191
1006
10549

6392
23290
3800

1555
4968
2802
6188
2972
5342
4944
3089
1489
5851
3724
4105
1646

1056
5939
2669
6416
1825
5364
3055
3718
4637
2092
2427
4896
2074

1146
2745
5553
12023
11630
2821

6444
9693
5025
6574
4847
5858
9920
10756

32182
22267

55301

6186
1515
9204
5359
2695
2760
4021
8269
3633

1682
3758
7745
8645
5672
4336
7765
7497

1099
1107
2501
3511
5559
3384
5934
5439
4723
3229
5376
5032
1133
1239
5806

15356
9800
3837
10024
13159

11534
6116
4343
8949
8628
3215

2118
4216
5874
7366
4894
3959
3906
4397
1945
4080
2564
2609

1559
1514
2947
6754
6285
6265
2403
5718
5167
2172
4605
5420
1806

1880
1014
4994
6146
4752
3350
3872
6534
6554
4005
3413
2201
6360

10572

5543
2387
2323
5565
4920
1185
3712
2674
5513
3267
2714
2453
5923
2020
3843

5594
7165
3281
2559
4424
3250
6706
3870
2600
3620
6853
6549

12197
8838
8398
7951
5818

6279

16622
3657
9950
18164

4487
1146
1953
5985
2255
7752
5966
6953
3238

8433
4943
3379
10418
8317
2119
2264
9044

25424
15342

4620
4435
1247
5616
4740
7872
6710
6494
6666
7926

2587
4186
4175
6572
4189
6265
3357
6427
1014
1909
1065
2211

2894
5557
3884
4400
6992
1825
4375
5124
1442
2902
1538
1557

6294
5991
8711
6323
3985
6012
8327
1353
5585
1514

3054
1488
1453
4860
5875
3625
7313
3609
2707
3936
4253

7605
10203
1976

64078

12356
7090
2556

1467
7531
5418
2366
1855
8463
6436
5521
2152
4088

9810
15885
8104
12905

7369
5389
3464
7901
7781
2875
7377
1927
3870
6763

3990
6074
3698
8562
2305
1623
8346
8680
7690

8271
7058
8607
9116
4157
7026
7873

16451
4437
19220
3783

23509

15774
19061
19118

8042
5973
7798
2217
4093
3706
1467
3921
5878
2791

9142
19544
16563
5584

6394
5320
3521
3956
1751
5318
5031
2838
3506
3504
2551
1141
4733
3035

15753
2123
14170
18870

5179
6821
6559
4032
1024
8634
2377
1570
4358

1640
8885
6912
3939
10328
10308
3548

5391
1362
1541
1614
3236
1704
4088
5284
2395
6504
2050
5511
6782

7842
1185
7244
9998
3575
5292
6126
3264

4962
1758
3553
3450
5493
5000
1515
1174
6031
1007
5762
5033
1080
2817
2795

5822
3591
5416
5487
2110
5372
6614
5136
6450
5250
4411
1851
6421

42358

8051
10473
11286
19931

9617
4786
2155
3527
7553
3911
9670
7243
7662

23109
25276
6310

4840
6771
7719
4482
6687
6191
6796
4313
7656
2828
1535

2050
2929
1517
6796
1070
6924
4229
6813
9252

2515
32597

4389
7467
11143
8271
3081
2180
9659

5420
2126
5522
5579
6024
3291
2180
5429
3860
3589
2089
1362
5777
4103
3124

7858
4413
7488
7306
7816
9150
8791
4526
3311

4226
1438
4937
2222
4612
4677
2675
7870
7259
5322
3063

5953
2931
1763
5986
4926
4405
1628
4778
5978
1446
1388
4629
5080
1781
5090

25276
8575

7338
5122
2243
3416
7198
4917
2338
6466
7689
2006

6879
4534
1218
10503
3970
4274
1144
4666

5315
4696
5183
7596
4734
3643
2167
5441
7878
3347
4204

2522
2992
4658
7624
4018
3053
1575
4305
2551
3848
5441

29831

5920
5770
1031
2102
5043
3067
5257
3209
3909
1806
2172
2880
3007
1567
4663

5017
4609
8767
9643
4702
8519
7904
4716
3679

1068
6794
7802
2099
7998
3563
6051
6596
7987
3732
7337

2241
13670
13185
6208
12631

29307
3839

11354
6015
2257
7358
7688
5290
3929

12228

3334
8211
5328
4224
5626
7635
7615
9511

8075
5712
7226
5213
7236
7907
1417
6890
2571
1881
6929

1189
2124
5640
4677
1487
3608
2828
2911
2102
2169
6087
4343
5689
5315
1946

7302
1351
1299
3498
2752
2502
7397
7139
1571
1882
4967
7024

2453
2774
12613
15345
3686

6213
6151
2743
3336
7888
7792
2768
4780
1782
4613
1344

"
    .to_string();
}
