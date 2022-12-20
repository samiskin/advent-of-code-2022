#[allow(dead_code)]
mod utils;

#[allow(unused_imports)]
use crate::utils::*;

fn main() {
    let input = _get_input();

    // ----------- Parse Input -----------

    let parsed = input
        .trim()
        .split("\n")
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // ----------- Solve -----------

    let shuffle = |num_shuffles: usize, decryption_key: i64| {
        let mut vec = parsed
            .iter()
            .enumerate()
            .map(|pair| (pair.0, pair.1 * decryption_key))
            .collect::<Vec<_>>();

        for _ in 0..num_shuffles {
            for i in 0..parsed.len() {
                let loc = vec.iter().position(|(vec_i, _)| *vec_i == i).unwrap();
                let el = vec.remove(loc);

                let mut next = (loc as i64 + el.1) % (vec.len() as i64);
                if next < 0 {
                    next += vec.len() as i64;
                }

                vec.insert(next as usize, el);
            }
        }

        let zero_loc = vec.iter().position(|(_, n)| *n == 0).unwrap();
        let next_1 = vec[((zero_loc as i64 + 1000) % vec.len() as i64) as usize].1;
        let next_2 = vec[((zero_loc as i64 + 2000) % vec.len() as i64) as usize].1;
        let next_3 = vec[((zero_loc as i64 + 3000) % vec.len() as i64) as usize].1;
        return next_1 + next_2 + next_3;
    };

    // ----------- Print -----------

    println!("Part 1: {}", shuffle(1, 1));
    println!("Part 2: {}", shuffle(10, 811589153));
}

fn _get_test_input() -> String {
    return "

1
2
-3
3
-2
0
4

"
    .to_string();
}

fn _get_input() -> String {
    return "

8197
1457
9638
1252
-3118
3637
3549
4684
-3054
377
-2045
-3590
-4687
-9673
3704
-7735
1172
-5589
-6259
6182
6238
-3086
7219
-3830
791
339
8311
-5577
7026
9298
-5762
7385
-6734
4599
3569
7706
9634
8785
647
-8795
-6687
-2736
8105
353
5383
790
-593
-1191
-6307
4830
6202
9147
-3323
7760
-1196
-3771
1469
9498
7851
1518
9067
1551
-4574
2340
5050
7039
-9817
4625
9101
-5744
-6493
-7386
-1807
6807
7507
-1294
4405
-9670
8946
-6475
-9233
-9773
-4150
1526
681
-1722
3641
-7320
7350
-1853
-5319
6405
1310
1123
832
1632
468
-5901
-684
6317
-8230
-6182
3057
-5404
-810
9077
8156
-9161
-5211
6514
9373
-6297
3042
6068
-2271
5110
562
-2418
-6521
202
-6723
-1585
-7976
-8994
-4210
5448
4695
181
5229
3070
8439
2676
146
-9732
-9807
1935
7995
-3546
-8534
6699
225
-8878
-814
4876
578
2199
-3370
-7422
7632
5642
8750
8812
-810
9661
-5381
20
6028
4549
4005
1099
-918
-9334
2161
1328
-3112
1055
5188
-3720
555
3411
7576
-8795
-487
-9533
7374
-9237
1531
-1679
2939
5398
1962
-1230
5833
-98
-9619
-4304
-9026
-7655
334
6200
-3494
-4868
-5325
5185
200
7949
-6828
641
7645
-5822
9785
-9732
-6710
-8911
-5434
2029
7532
-7231
3008
-6727
-530
1044
8554
-615
-1396
-904
-9489
5014
-5963
-6579
2396
-4525
6207
725
1390
5060
9541
3368
-612
-5093
-5936
1519
7717
-9375
-3716
8820
5481
-3937
-3035
-2902
-204
6915
8568
7782
-8073
1916
-1434
-1399
5457
9771
1772
2062
-4527
2277
-6489
6377
-5867
-7191
-5483
2852
3411
-2733
-7187
4593
-4858
-9513
2438
-1313
-387
-8379
-8245
1252
9437
-4137
4329
7351
-5343
4389
-7053
-6493
-8192
9982
408
-316
7997
-4125
7579
-9940
-4186
660
4346
13
3468
-8222
6732
5012
7199
1513
4969
-179
9841
-3171
-3192
9818
6323
-9016
7522
6187
-9648
-6613
-2085
931
-430
-6431
-5132
-4367
8877
-7249
-6802
-3314
-2090
-4985
6935
6928
-6906
-8757
8424
-4137
-5538
5876
-4466
-5773
-5359
424
-1759
-8943
3469
1640
299
-8639
-5542
634
-9948
5787
8234
9465
-5858
-812
3543
-2744
7373
-3373
-3152
-3980
7278
1326
3926
3987
-9734
5310
9771
-5531
-2900
-8943
8785
4137
863
8820
9768
6754
-8501
5348
-9159
766
-5085
2161
-1247
-7124
-1947
7039
2861
-8830
-9182
9610
-3075
3639
-5936
-5262
9811
-9931
-1698
-9518
4141
8550
8869
-1041
3315
8002
-6781
3400
7051
-7650
843
-2810
-8397
5593
-6293
-1204
-2138
-4862
-2349
-9916
-2288
-3948
-3410
-7936
9615
2151
2415
-7345
5014
-7104
7384
4875
5987
-7397
5059
1263
-7897
3380
-4587
3886
2199
-2505
-5180
-1460
-5537
9227
-8192
-8558
-7107
-4555
-1245
9360
-7283
1780
-1018
1098
-9166
-32
-1987
432
7198
4208
1274
-5491
7140
1239
2645
-1211
1554
2340
-5305
-5886
-4625
-1550
-89
496
5905
-1834
-3999
-8276
-6543
210
-4212
-7135
7223
5966
9282
8919
-3311
-7063
8020
-3763
-1565
2925
-1383
1025
-179
1273
-7134
8623
9967
-882
1904
-6682
5635
7614
-6532
-898
3764
2818
8696
-1330
-6963
5367
4682
4942
3211
6414
-5735
8945
-5391
-8572
210
-3269
-3518
-7191
256
-8121
-1330
-4467
8368
-5130
2254
5591
-3869
3100
4798
2203
-3324
-5981
1912
2530
1283
2987
1374
-2285
1105
3532
6505
-6313
6788
7121
7325
4682
1644
-4931
-2045
-1998
6140
-4908
560
-3225
9486
-5669
-5605
-7292
-6890
-4466
-5428
-6300
-5991
-2888
1423
4490
8561
-435
893
-354
8260
2131
-3233
-6263
89
-6238
1276
5860
-23
-2508
-8380
-5929
-2558
4033
-5306
-458
5832
4055
-6070
9348
9890
-4588
9058
-3118
7519
-8245
9587
1804
-5642
647
-8329
5331
-1242
-8084
-2434
1766
-5618
8807
-9759
8569
670
-9654
650
-7362
-4964
-1245
-3770
4949
2762
950
5397
-2006
-5524
5884
-8694
6199
-6163
-9688
-3136
4833
2667
6417
-5520
-1124
-308
-9375
-7993
6916
-3567
4192
-2738
-4806
-1328
-2513
-6128
-3161
1819
7528
5232
-3673
-5011
4176
8079
5917
235
-9762
-354
3622
1252
7152
4605
-7538
-5983
9592
9932
776
-7352
687
-9458
5331
7616
2839
9684
-9760
432
7614
-2192
-9632
1774
-8515
9276
8397
8209
3733
952
572
1763
9692
6767
-7933
1272
-2728
-9510
-2546
-300
8406
-5531
4140
-9361
-9078
3682
-5768
2345
-431
-1361
9323
2057
-3004
-5991
3257
5241
228
-7021
-8835
4748
-2496
-31
463
-2926
8829
-6112
-3645
6696
-7788
-9543
-7385
4482
8567
-7820
6276
2013
-9329
7049
1780
5341
4508
-7910
-6701
4044
-5848
2225
-2384
8714
6456
-5306
-8212
-1752
-5809
6141
-6717
-7237
3089
2979
-594
1819
-3017
9973
4508
6839
5551
-6182
-499
-4224
-3144
-1010
5977
-9611
-3891
4738
-5380
-6256
-6295
-4881
-1285
-7793
9672
3042
-1374
-83
-3080
-9489
-8263
-1191
-4117
7739
-8438
3087
-810
959
-6315
-3843
8260
6395
9939
5612
-4171
-3083
-2620
7277
-183
2901
-5410
5784
4859
-1478
5721
-655
-7862
2091
-4532
9608
7385
6728
-3539
274
2144
8328
9825
6607
-2813
6352
8563
-4806
5310
2556
-92
-6174
-8559
-6481
9375
-6789
-4092
1289
-1920
1697
-4625
-5786
-4270
-3806
9679
7981
-4646
8015
-7251
-553
3173
6696
-8065
8482
-5576
5403
-3458
6461
4600
8680
-9514
9880
-7676
-5556
5522
4423
-7227
8116
5448
609
-5357
7201
-6373
-6128
-9183
-4996
6151
-1129
3201
9237
957
4228
-8708
293
-11
6985
-2705
-9960
-3473
-6895
9669
-6931
4633
7767
3518
-5784
2181
-5858
2640
-3650
9776
-1973
4344
5195
560
3963
9290
-5964
989
7273
-6724
1652
-4122
6647
8104
-7728
-939
1328
-4243
-2298
8202
-9760
-9787
8807
-3300
2076
9012
-5963
-2238
929
8585
9166
314
-3189
-5577
6288
9370
-4134
-2293
9515
9346
-8222
-9738
7515
-2282
8602
-3770
-7634
-658
-3144
-1447
5978
-4292
1660
-6590
-5541
-2908
9042
6849
-5076
6640
-499
-899
-4401
-7473
2846
-8753
2352
-3703
959
8207
6102
-328
-4405
-3969
-2300
-4883
4247
6080
-8472
-7559
-5260
-5460
-9902
4443
-8192
5239
9487
9813
-8515
-361
-3682
-9763
-7105
4455
2556
6914
-5632
-1102
-3154
-8541
-8311
-3778
7505
-5463
-1289
-2107
-2527
-8857
-8070
3411
5527
-9159
-2475
4295
8598
-7376
8405
-756
3903
5711
-1952
-2141
-1555
-1722
-8011
-7524
4066
7505
3313
-7143
572
-6703
8705
3476
-8609
-1929
-9738
-1010
3057
9268
-975
-6432
213
9728
7696
-2361
6640
-2570
-7249
-2825
-9916
-6872
7502
-8765
-4941
-6499
5501
-4522
4421
-3680
-8292
-1005
5277
6966
132
4673
8928
-8725
-7618
-3666
6043
-3049
3211
-7620
2439
-3277
1641
-851
-1473
-9938
941
-3353
9597
-2261
-3221
604
1792
5998
-8262
-4447
-8006
-6676
-218
2568
-1085
-6639
1519
-7021
5241
-9159
-5026
-6165
-163
-431
648
-7520
2365
1131
2694
652
8479
499
-7163
-8532
8544
-7268
-7205
-5384
1175
-6156
-4683
8386
-1739
6447
4818
67
-5901
-4774
-4155
-5672
9067
6344
-8490
-8124
-699
1899
-3740
7735
-4774
-4356
7632
652
-6892
-9819
8932
1803
-8652
4716
4301
-339
-2270
5305
-8294
1423
6699
1148
-617
9419
-777
-8750
-218
-7637
-1338
4781
-6481
7987
-3734
6562
9510
-2374
-2299
-6462
8517
5654
2224
7789
-9770
-7566
7876
-8766
2038
5551
4064
-1624
5857
1888
2198
-266
1756
-6687
-2453
-1930
-1464
-8240
-9342
6319
6946
8560
-2821
-2466
-3331
-8376
5200
9573
-1136
2733
-9755
-9049
-3402
3591
-2351
-3599
-33
151
9187
-3062
9657
2392
3532
6711
-8987
-687
-8933
323
-6094
-4555
86
-7655
9998
3183
-705
8260
-7709
4907
3698
3374
1395
9106
-4520
-6728
241
-9926
-6024
7120
-6771
-2761
650
3660
-2821
-4447
3925
-5348
4422
-5746
1740
6000
-5832
-1998
-1258
3724
1918
-5214
2568
-4073
-8358
-2008
2280
-8062
2057
-8043
1526
-9117
1845
-9656
-7897
-431
3349
5985
1783
8901
6483
-637
6119
6568
8231
6941
-8101
2983
5730
-5262
6806
-1490
465
7222
2788
-7266
-141
1380
1345
-3105
439
-1849
-793
3287
623
-1074
7384
5207
-316
-7263
3313
300
-5021
6914
2247
-4921
-6297
-8859
9875
6030
5321
4811
2901
-7946
-8228
7225
2662
-5129
-1746
-5261
-6760
1752
8892
1819
779
-3229
8066
-2653
-4949
696
9865
3988
-9412
-4572
-4683
-9619
-8765
-4281
8799
-5811
2131
3819
6920
-3234
3799
499
-3838
2847
3183
5383
-9666
-7237
-3272
-2080
2202
-1902
3660
1380
3298
7603
5905
4269
-5206
-1901
-1121
5007
-2127
-2902
4852
6102
7202
4885
3563
9234
-8526
1085
2663
-2437
6778
-638
6586
6304
-8399
2516
3748
7463
-3833
-1009
3532
-2881
-1039
-1441
6685
6996
9939
153
8932
696
8082
-7644
-4019
5286
-2026
6456
5298
-3
9117
-6086
-722
-820
2935
5840
8116
5295
-8449
926
1260
7309
-8486
-5748
3139
-2353
-7163
-7452
-8904
7904
8341
4344
8106
5708
-7803
-2412
8392
4111
-2261
5600
-2324
-6613
9748
-4617
2403
6898
935
3343
-5692
4315
2529
4766
-3988
-5067
9400
2682
9252
-6970
6442
-8683
1085
1034
6446
8805
6463
-5497
-3507
-2821
241
-2639
5549
8341
-3765
6025
-9153
8206
8406
-9680
3360
-9467
-1969
-7268
1345
-2446
-8766
-3567
3126
4013
417
-9161
513
-2752
-4153
5592
2616
-2669
4257
5242
-3339
-3233
-1989
989
2529
210
-3286
4593
5059
2013
-8653
-8563
1540
-5113
1423
9406
-1269
-5518
4474
9778
22
5943
-6441
8043
5035
-1330
-5319
-4973
4365
1641
9363
-350
2247
1772
-8703
-7134
4833
-9670
2050
8082
-7030
8978
7749
-8398
-4397
7122
-83
9771
8565
-4728
5648
-475
3949
-2776
9003
-9352
-9317
-2349
-3771
6968
6539
2237
-4491
4769
9819
4033
-6247
831
-9619
-9123
8750
8065
-9026
-8663
3411
2811
4751
5050
-8920
3350
3710
-9819
9672
-3919
-2464
-1070
-2771
3160
5102
8689
-9401
-4684
4747
-6635
-1490
-7147
3553
9848
-5846
-6533
560
6988
-9513
7934
2922
5978
-2710
8482
5205
4539
-1325
-2680
-7793
7773
-6453
-5132
-8208
6017
-4985
2865
5278
4360
8072
-8874
-7146
-7263
-6125
6519
7398
4686
5240
-1956
-9874
1977
-2554
-7164
-9449
4013
-6226
3211
7921
-5261
-4386
8447
-5503
3604
589
-7566
-4
-8343
-4217
-2376
-3854
8544
474
-9473
5841
1667
6166
-5404
-1146
-9387
-9481
-5998
7727
-3325
-6238
-9159
-1361
-7854
2238
4375
-2572
3553
-2280
727
-774
1368
-6687
-8599
-5130
-3165
8424
-8999
-981
2565
-1746
-8788
4119
-5121
-3674
-5646
2958
-9439
5068
-9063
4757
-8741
-2418
-1000
-9619
9818
7429
9094
-1959
-8365
-6714
3246
7949
-7616
-4426
-5640
9147
4360
-6638
-6721
-9277
-614
-5542
-3294
3751
-4199
6066
6463
2199
-5550
-5264
-7473
1928
-684
-6118
8463
4649
-7078
1315
-2374
-4196
2079
5072
5857
1602
6268
-1164
2069
3948
2422
-5403
-9277
2144
-3144
-2080
-2676
-1620
-458
8919
-2778
-9776
-8533
7757
-4199
3991
-6393
-1041
-8397
-5936
5860
-2466
-8543
8874
-7175
3691
223
6622
1263
634
2922
2837
-1365
-4182
-1126
185
7489
-1005
7242
-6375
-8074
-1318
7945
5359
6926
5240
-6714
-7104
-6354
-658
-1274
7816
-9737
4307
-6035
-7537
-5691
-1546
6199
6260
1697
9931
9522
7188
-5748
-7342
2405
665
9299
348
-481
3139
692
-5362
-9467
1256
2144
-7237
3143
7809
5111
3560
1753
5701
-7694
2117
9411
-8851
-481
9407
4394
3577
8948
156
-9879
1780
-5437
-7223
724
9895
5010
7353
5303
-6682
-4243
3405
-9350
-3300
-9545
6843
-4020
-2475
-2577
643
-1447
6692
-4053
-6950
4619
6550
998
3757
-7981
5660
6648
4638
107
6151
3106
-7585
9927
-8278
2677
4050
8841
-40
9167
4301
-7995
-9663
7852
4093
-9383
2892
7615
-2564
-4934
2196
-6282
1827
4947
2939
-7349
-4873
-6383
-6462
9490
2723
-6919
3698
-7203
7140
156
6505
7958
-2202
-2080
-5423
-3614
-6799
-3634
1978
-2609
6089
3255
-525
-7557
-3504
-3590
-5538
-5718
1389
5564
553
3682
6480
-639
2796
2750
9380
-9577
-4237
-3016
139
-1338
5556
547
5174
6475
3361
9209
-4082
-5963
-5660
-2483
-4666
1789
3797
1025
-2120
7141
1556
-5827
6615
5070
3527
2551
-5642
-638
-7381
6275
-1345
7050
-8848
-6704
3819
3260
-9772
1089
878
7509
7605
-6660
4162
-3030
8503
-3375
-179
-3951
-7447
-7686
2006
-2643
-8508
-7361
-163
-1125
5316
4663
-6089
-875
-4653
6194
-3457
-3740
9982
5803
3347
-8643
2641
7049
-6461
506
-9781
5352
-2664
5730
-9510
775
-398
5660
7442
-3886
5593
2087
-5416
-5946
-5130
-3161
7098
-1805
2038
-36
-4139
-8073
7773
852
7395
-1603
6293
4905
-5357
4780
1765
5014
4584
-8327
-2747
-1354
7152
6986
-2171
9898
-1198
8851
9748
-183
374
5222
-434
-6425
5555
8755
4237
402
4137
-4520
7567
-6682
-2858
8139
8772
1408
3246
4000
-7159
-7473
-6433
-6404
332
-3973
196
5745
-9582
-2604
7945
5559
9376
-5048
-9489
7647
-6697
8680
6275
-1986
-3893
1977
529
-7185
-7697
-4517
-344
6811
1112
-7059
-862
-638
9720
8471
5905
3349
1860
-4036
9875
1819
-8308
-2881
7614
-4159
-9373
4308
71
3988
7949
-7180
2757
7691
8336
-7104
-4377
-6425
5900
-3221
5988
-7359
5442
6433
-449
-6152
-5686
3094
5
-5011
4373
1105
3532
6532
-1214
-4466
8634
-2455
1331
2003
-2735
4524
3892
-8528
3518
4771
-4155
-1282
3918
-2708
-4673
-8253
-8579
-5132
1132
-9166
4130
-367
-6471
4447
-5090
-1355
-771
-2087
-7381
-4489
6571
-6684
-6574
-7713
-1058
474
-5428
6522
3948
-6392
9786
-7118
-9171
9771
-7934
1080
6317
-5348
7307
429
-987
-220
-1216
7848
-993
53
9881
5102
-6854
8370
5600
4913
2526
6875
1034
-8116
-2134
6779
-6152
9776
5285
-8854
2895
8719
-7262
2364
7589
4396
5863
-6521
4653
1574
-4730
1632
2749
-4155
-8220
-4569
4860
-2908
-756
6573
4439
3830
3569
-8001
3531
7481
7778
-6687
-8082
-1148
4460
-687
-6471
9182
2546
-5499
-9817
-9159
3195
998
7224
5900
806
5070
-8716
-8757
-4218
-205
7613
-432
4279
9449
5555
-499
-1053
2196
-2936
5367
2305
324
4741
-1935
377
2290
936
2278
3807
-353
-1673
-3554
-2151
-1394
-7720
-599
-2418
-1544
9228
6187
-9280
2811
-810
-4365
-5692
6334
6392
8839
1766
2400
1632
-3518
191
9776
5691
8269
-2220
2553
-5508
8962
8823
-1516
-5385
3898
-1930
-4059
8590
-4839
9528
2396
8554
1904
4319
1389
-4663
4724
-4392
-2862
-5088
-7147
9623
9945
-3368
6843
-7318
9322
-3241
3468
-7773
-2666
-3887
7505
2257
-9759
7373
-1727
7783
-7474
9531
3444
-368
-4449
-7105
668
-5127
7498
7869
-5810
-4643
-6993
6795
-4844
-6308
5876
-9153
7949
-684
-1233
-1051
8463
-1458
-2677
8386
8085
3547
532
-7038
7378
6647
941
5366
4796
3400
-6086
-7780
8660
2078
-1698
-440
2765
92
-1350
1387
-3905
-4155
-4705
4756
-9107
-4569
-7081
-4963
8270
2350
-1235
6584
235
8639
6002
6649
9026
-2639
6477
-5642
-1383
7110
-3532
-5416
8530
-8793
-3902
-9915
320
-3417
1526
5501
-3666
2014
4807
-6295
7784
488
-575
-5088
-6837
-7248
373
6618
-8633
2935
-9063
6186
8137
6729
5877
1236
-7864
4978
2425
-4196
7940
2529
2709
-9929
-427
-3491
-1993
-3110
9725
5149
8580
-1164
-3101
-7870
-2445
941
-3045
9303
-3146
6911
-3030
-2350
-6695
7442
7714
2775
-6037
-4873
9241
-7090
-990
7748
-7827
2665
2597
-6572
-4083
1873
7229
5664
3910
-3168
5287
-3531
-3410
-8943
3578
-9990
-9036
3737
-4419
-5076
671
9890
-1097
-4614
5708
-3266
4850
-658
-5216
-1499
7725
418
-5153
6914
-4221
-7158
-7102
6828
-6893
-4892
2686
-4897
-1401
-5217
3932
-5532
-6721
9625
-8101
5611
4779
5505
5394
-7859
-3832
4307
-4419
4224
-8315
8607
6851
5456
5419
5289
6355
2013
7835
-463
-3346
-7612
6255
3374
8895
-7959
9946
-5631
6417
4596
8639
-7538
-2571
-8052
-3464
8766
-8311
9010
-8650
8755
712
-4310
-3771
3258
2709
4570
2514
-8590
-7855
-6694
1947
-3105
3591
-4135
-8435
-8240
-8904
69
1977
2178
-6094
4180
-7792
-1191
146
0
6066
3621
-5132
-9812
-5397
6299
-5039
5185
-3843
-8186
-5362
9423
4810
-8698
7218
2922
5690
-547
3467
-3650
-2811
-6453
-5211
-4673
641
1149
8018
5316
-3044
39
4221
7635
5459
5212
-238
-7858
-5091
-2775
-5983
1272
4430
2127
9761
-9366
8365
4197
-6682
8393
4064
-4429
-6175
-205
1249
-7643
1217
-3375
-4118
-7697
-9266
-7232
-642
6693
7945
4599
318
-3716
-3054
-8211
3021
-9401
-6487
-2110
69
-6143
-8770
9579
8805
1773
3591
-4845
6142
2180
2257
-7085
-6431
4195
5478
-7497
-5362
6352
2925
3692
2776
560
4911
6957
7517
2144
-3956
-2717
-9745
-8228
-4591
-1760
7113
3412
202
-2789
-724
-2849
9565
3122
-347
-7831
265
562
-9760
2468
6926
-6129
-2005
9710
-3194
-1494
-2381
50
-8125
-3499
7541
9363
5746
-1396
7050
5087
-9721
5809
307
988
-2308
7242
-2024
6292
-3379
4086
-5399
8058
-9695
-9272
6627
-4630
5382
-4555
8512
-5687
-9690
-44
2224
8482
4229
5730
-380
-8708
7078
1501
-8736
-1184
2755
3860
-1576
5470
5394
4802
-9094
5028
4624
-5661
2072
-1283
-1006
-6239
5248
-9730
2014
3386
-2621
7305
9516
-952
3802
-7493
3583
-6618
5047
-9938
-3027
3223
7586
7998
7615
-2538
-5611
-8439
7749
8137
1981
3293
2744
-5951
9165
-5275
-9920
751
2711
-1831
-6511
-1723
7309
541
4076
1960
9483
-6835
-3716
4440
6914
-8342
-41
7373
-6694
-4441
6459
-8755
5853
5441
5415
-4514
6061
-5809
-2888
-7766
4389
5238
-431
-83
-2030
-1243
-5132
1588
-562
8100
-1689
-4153
-1734
-1092
5375
-1779
-8228
4850
-7933
2744
-1574
-775
-2150
-6067
-7146
191
7034
-1868
7739
-7133
-232
3224
4508
5937
-7834
-9313
-1930
8796
-3255
-2429
1263
-9155
-3368
8571
5605
6817
4229
-8292
6887
-9401
6864
-9416
3846
-1467
4913
3691
-4040
-5979
9006
-5751
-6277
2324
-2134
-5415
4895
-8275
-1490
-2735
-3241
-3027
-1224
3841
-4602
-1382
9835
-896
743
4662
9877
-4185
9681
-8211
7750
2146
-8639
2731
-36
5073
6640
-983
7609
-2298
5863
-8866
-6094
6526
5137
-1549
-8139
-1129
-3567
-6682
5185
655
4584
7489
8231
9423
-2080
9579
6584
-9902
4830
-5773
-8368
2873
-3574
-9757
6725
6930
6078
-6181
2386
4396
-1722
5768
-8001
-5243
-8066
-6142
-7362
-7992
6221
8519
1857
-7792
7838
2151
2682
9552
4225
2979
-5751
-6371
-5982
8809
-7146
1804
-6550
-1585
61
-3630
-9690
-3045
-7269
3783
33
-3520
729
2352
-2708
-6628
-5674
-3083
9531
-2538
1180
-2002
8890
8209
-9049
-5858
-7852
3171
-1175
-1295
3141
-2067
-8825
-3586
-6550
-8291
-1968
4705
1922
-1298
-221
9537
-7358
-785
-6022
-1053
-3609
-3901
-318
2217
-3178
-9737
6456
2439
6158
3618
2598
-1420
8719
-1868
-4022
-7803
5757
-6659
-7946
3926
4625
-2423
1353
-3911
7789
8334
-2418
5257
9902
-1063
-318
8054
731
908
-8593
-1689
513
4313
-3796
1243
-6701
-8032
294
-8435
1412
5817
-8346
4077
4277
-4984
2132
2257
-7123
-8623
-3625
950
828
5840
2943
7122
3956
9391
9694
-7024
-5510
6681
-9085
-3027
6828
-6511
307
1283
6645
5148
3407
-1134
-7250
5060
2562
-5367
2396
5602
-8879
9354
7626
6536
-791
1176
309
-205
-4779
9239
6137
4649
6019
-6957
-8736
4038
-2620
-723
-6515
3905
-5920
3636
-9062
651
91
9964
-4934
2796
6839
-8370
2466
393
-3371
2006
-8455
-7994
6460
5366
-2354
298
-8188
3725
1515
7416
-5307
-2811
-6696
813
5730
970
3394
-1164
1966
2199
-8508
-3110
-300
-1481
9401
6973
-9000
-6432
-2946
4661
9813
-3040
8517
5587
5339
3998
-4274
4977
3193
765
5277
-5570
3645
5239
-7104
-4018
-4628
-6259
-8263
-9161
3749
5381
-5211
-3833
1956
-5157
-5617
-9840
-8615
-9800
-2482
4237
-7749
-5846
-8764
-4527
1667
-3968
6376
-3368
-3673
2643
5240
-5872
7716
-1527
1142
6643
9506
1966
3985
2271
2400
2749
-7134
-7738
4009
-3647
8565
7158
9264
97
-8976
4077
-9803
4368
7904
6424
-6694
392
6183
-8167
8705
-430
-4226
-7852
-9195
-4569
-2098
7879
-9678
-4053
7242
3826
-3192
-5572
2009
-1985
-7023
-9908
-2178
-5478
2165
6032
-8332
-2203
-4085
6463
9153
7646
1804
-748
-6425
-3647
-4245
-8278
1847
-2475
-7822
4532
-804
-8269
-8079
-8507
647
-6781
-6597
-8228
3784
-5556
-8508
-5111
-7156
6640
-6198
-7922
-2495
3075
-3144
-9467
5591
-3567
6049
1949
767
7662
-9390
-9863
9510
-593
-4030
-6354
2199
2334
7675
-3832
-3099
-5391
770
-5510
-9469
-9569
-8994
-3458
-8318
-9835
-1576
4850
-7320
-8709
-1721
9313
-5244
-5311
5634
-7154
-344
7647
-3300
-9965
2757
714
-921
6679
7441
5120
-1106
-1520
-3903
7197
-9221
-6063
2648
-339
-2896
-4883
9131
-6076
5733
-5085
9142
-431
2721
1761
8683
-8304
5306
743
8552
3140
-3232
6021
-2968
-78
-229
7732
3191
8065
-2056
-8036
-7320
-1421
9623
-9550
687
-4365
-2430
3257
-4873
-4365
-4292
-600
-3864
8430
6142
2352
-3350
-2717
-4469
-3461
-1968
3003
927
-3510
-915
89
6
4501
3037
-9689
-2814
7700
8890
-4801
-8221
9105
5073
4850
3068
1306
-1283
-8811
-7567
5668
7511
9623
-4676
-6259
-3278
-7800
681
3768
-321
8899
-4324
8059
9889
-7894
9874
-9500
8046
-4331
9257
-4920
-250
6711
-5483
4295
-9240
-5460
5120
8245
1833
7714
-2492
9608
-204
532
-8534
-6493
9776
-5583
8652
-8308
9629
-4427
-6854
-8319
-1357
-7228
1556
-1273
-2107
-5998
1389
-9593
-6846
6123
-8432
1877
-76
9155
-7805
3458
6973
-3027
6347
-9165
3304
1942
-1214
-7618
1465
5156
-6944
3146
7714
3556
2869
4747
-7438
-6457
6296
3948
3520
-757
4684
-2514
-7336
-3808
-7905
8115
9634
-9950
3207
-6497
7767
-8876
6729
5478
6248
-8361
-321
-7018
-964
3180
-1046
-7216
2587
-5929
6640
9566
9082
7154
-1844
8919
8040
8483
-3345
-4492
3151
-7460
1717
-5838
-2508
-2621
8447
-9529
-9375
7809
3158
652
424
-5260
-179
-9467
9995
6345
3693
-3854
8778
-4585
-4585
-9903
-6847
3454
4779
-8703
112
-10
-2005
6536
-1425
2059
6640
-6437
8082
-6136
-3105
-7713
-4146
-9539
-2462
-869
3343
6257
-3647
7614
-8730
-9322
1061
7087
-2754
-8208
-2508
5790
9830
-4489
8439
-2810
-5809
4340
-3339
9628
8836
8742
-2821
-2296
4142
-7898
-8863
-9337
-869
2063
4826
9716
7154
-4969
-9993
2723
996
-2902
-244
-3852
8351
5882
772
8331
7888
-4397
2928
1262
-9309
5886
-1673
8577
-3141
5417
-4774
7345
5457
-3554
-2261
-2580
6439
-8515
4127
-4980
7373
2075
-2008
-1175
552
-3052
-1045
-8947
6441
2349
1587
-163
2290
9762
-1689
-6789
-9643
9830
824
-2665
-1540
5073
-1859
-8490
1598
-9481
7846
9486
2235
929
-1392
8785
-9075
1943
-9228
5903
9675
1239
-4630
-7332
2381
-8441
1423
9086
7098
-9920
9412
-3683
-1606
7728
4850
206
2057
4435
-2453
7353
6926
-6063
-5853
-9383
6479
-7237
5075
-5765
4323
2591
-4504
-8339
-8533
-4444
-2492
4435
-2857
1405
3361
-8065
-7924
7546
-1403
-2992
9776
440
-2033
-7251
-3129
9998
-8856
-7175
-7634
481
560
236
-5518
6920
3096
-4873
-9131
-2786
1626
6988
-8593
3349
-7320
7890
9281
9506
-3237
-684
-6521
-6427
-4465
-6427
3195
-663
-5089
4951
-9127
-1721
-3622
2006
-4427
-4988
-3703
-6392
6570
-3857
-112
3210
-4544
6645
7809
-9727
3161
210
-628
54
-5870
8058
4577
5670
-1396
-8611
5549
935
9982
-5132
-7055
-1016
-6799
5052
7840
2629
4964
6621
-5089
-2667
-4274
3250
-851
9283
-1481
-3057
8636
-8757
3898
-8144
-200
-4122
6147
-2418
-6205
1071
-3327
2759
-9817
-820
-7154
-4969
3760
9004
6914
4831
-1487
-5148
3115
5477
-3771
1142
-3765
9186
1224
5156
-7611
-2006
-6470
4373
8977
2806
6777
1978
1085
-8818
-9819
-7977
2167
-8999
7712
-6950
-3750
-7648
-7436
-7146
69
-9670
1988
-7143
-1351
-1969
6019
-268
-9538
2817
2403
2576
-8393
4253
-458
-5211
-5531
-4598
1803
7242
1126
560
-2356
-1935
-6315
-6877
8785
-7577
9678
-3269
-2378
9469
-2085
8622
-6983
-3331
-8952
1710
7576
-3949
668
-3366
8750
2151
3099
-5261
2199
6226
6565
6994
9217
-4848
6058
-6431
-2777
-8533
-8228
-5416
-5331
-1409
7759
-3554
-3403
6828
-8145
-9513
7635
9003
6790
-8365
2428
2396
447
5189
418
-3218
-5261
5252
3949
2328
-9838
-3349
7049
5457
1252
-7603
1770
3135
3183
-5748
941
6575
1529
-7097
-7476
2591
6506
-872
-7433
-7805
9568
-5460
2733
3291
562
-1917
-7347
-3977
9281
2922
-8496
-3154
-5979
-8332
3896
-4395
9564
-6987
-4073
-5574
7645
-9694
2049
4778
439
-4053
-4411
-1902
-6383
-7953
-9541
7934
-2006
1918
9593
274
-8739
-267
-2543
4287
7431
4983
7121
-93
-4436
-5168
-3320
-2720
-8380
-3353
4498
4589
5987
-6103
4037
9590
4180
-205
9625
8839
-8757
7846
8487
-4902
-8188
9322
-637
-9040
4860
9639
4876
4605
7411
4649
-2793
6985
-6427
1541
-1546
-526
-628
-6934
854
6282
-1189
5295
2580
5703
4862
9030
9423
3932
3591
-4878
-8766
-9948
77
-6297
9023
-5762
-2024
-4563
-8449
3345
-8315
450
2625
2428
3102
-576
-9539
-9729
7749
-5973
8058
-2754
7422
-9848
8337
-7003
9475
-5355
-4569
6058
-6352
-3454
2591
4093
-705
-7994
8600
-8699
-3518
560
-5949
9118
-542
-8472
-4873
-6559
7028
7429
-1085
8567
7270
2199
2529
-100
9597
1930
1457
-589
-383
-9441
-5225
1782
3842
3430
-6433
6227
-3535
1583
9564
3332
-8611
-8464
-5061
-3900
-4462
5705
-4237
-1554
5157
-8208
-1586
-3150
-3074
7795
4851
3447
-2088
2290
7940
-665
4686
3541
-1452
4736
-2821
9737
-1102
-4447
6994
4716
-9734
-6186
-6814
-2439
-7249
-4441
4655
7143
-4873
926
8841
-3154
4329
2652
7481
836
-881
2706
5180
-2325
368
1405
2524
5310
1690
2396
3932
3624
-9902
-8212
7083
-7835
2422
-1675
-4671
7345
133
6021
4300
3846
6392
5171
-7078
2744
8386
-7728
4431
-5707
8832
-4406
9348
-6521
-3485
9350
-3776
-9481
-7381
9939
4833
-557
8079
7118
7712
1544
-9849
-3702
9982
-83
6293
788
-2088
-1594
2152
432
-7219
-5378
-3972
-8979
52
-3165
-9833
235
1702
480
491
2694
2930
7484
1816
-2658
-7187
8428
4479
-3233
6356
3905
5459
9214
-3443
-8764
8054
1668
751
8639
8046
-5987
-2010
7532
6681
-8259
-2329
7484
2105
4733
3407
7984
8512
5436
-4755
1262
9573
-1614
1300
-7956
7750
-8866
7622
-5391
8025
2604
4783
-7627
1691
-7810
3311
4951
5893
-5091
1763
481
7449
-4942
-6950
78
-7081
2103
-1649
-4980
3194
-9998
-6444
-8145
-4764
7881
8018
200
-446
-1956
7696
2024
-5048
-4552
-7134
7628
6961
-324
3419
-2772
630
-2518
-8733
6808
-575
7928
4375
-5177
-1034
9725
-939
1884
-5861
-2928
3350
-6809
3988
-3215
1819
1624
-3239
5241
-5076
4673
4474
-1129
3918
-2939
-400
8511
-2250
-2088
-8441
-3937
7507
-6182
1873
-6182
-2067
-4268
6443
6719
4344
-3458
-2324
-4560
-3875
-9752
1312
3600
7381
-3259
6140
4550
5241
8526
-3378
8932
-1458
-1
-7438
-8471
-4746
2202
2895
-9856
-93
-6772
-9967
-6236
-2841
-8699
7579
8423
4150
-7081
2749
7737
-4881
4945
8644
-7427
-5231
3645
-2771
5985
-6600
1059
4394
-6076
5903
-5728
-220
-2633
-8639
-5657
1607
-517
695
5214
9361
2247
-6532
1180
-4107
-4938
-4915
3298
2053
-8979
3964
-2571
-4934
6796
-3703
-7807
2861
7949
4315
-4544
5283
-8378
-7284
-8032
-8202
2762
-3869
-2219
8834
-5323
318
1273
-5674
481
-6113
-5307
-419
-6008
-89
3493
8460
6865
-6145
-2821
-435
9405
5157
1534
146
291
1123
-6499
8550
8874
823
-1396
-4973
869
1262
-6493
1984
7644
-710
-9545
-9270
-7560
7742
106
-4656
3330
-8380
-4882
-6946
-400
577
-4122
-4427
5721
2901
6911
-7328
5660
2358
2398
-6597
4370
-5155
9817
92
-6940
9241
-6183
8345
-7120
-4965
5205
-3009
6606
8506
9661
4911
7954
9361
6988
3643
9553
9308
112
-5796
6405
3703
-6688
-51
-818
-3932
7925
9946
-8765
-8579
2419
-7485
-1215
8682
-4056
4770
-3089
-435
2376
-7671
-5534
6692
-718
8607
5300
9590
-3681
3543
-3230
-3905
-4881
-547
-1113
-32
-8728
-3905
839
5952
-9872
-1243
-3465
-2720
-4569
-1871
-4969
-1900
-5531
5363
-7484
-9905
-3702
-8441
3124
3981
7473
-9422
4041
-1352
9470
-2391
-350
1551
-5537
4431
-1429
-935
-8257
3532
-9161
6677
5519
-8564
2417
-4721
3094
-3133
5539
4212
-830
-9404
-8611
5711
-8304
-1948
5687
-8489
3115
-8750
-4720
6275
-112
-5872
-174
6548
6156
1838
2144
-2153
8809
-9582
-4848
7547
2298
9474
-629
7686
5682
-5246
2111
-1394
-9940
1272
-1447
3349
7430
-6741
5309
1486
-4205
-3568
8308
9433
-3535
5997
-5946
-7746
-7436
-727
5167
-8913
-2205
238
1004
5878
-458
-5132
4625
3830
9540
4230
-1860
7840
-9945
2630
-6293
-4234
7049
-7527
2132
-8370
-4245
2659
-6714
8796
3516
4482
6421
-7359
8045
4286
7792
7873
1148
2943
-3027
-4527
-9268
7147
6573
-2020
-7564
-2280
4315
-267
-3952
-5954
7154
54
7840
4378
-3485
-7422
5905
6310
-6847
-5700
507
-6057
3723
8484
-9429
-5872
8189
6640
-2232
-210
-6403
3766
-7143
-3772
1567
-4155
8441
7280
-9441
-5945
-9145
4140
6459
-9133
1689
-8008
5355
1877
8544
7345
836
5490
-9670
3141
-6808
1389
-6103
2610
4326
9654
4605
-3154
-2784
-2902
2428
-6507
-6129
8810
1667
6991
4139
-9288
5887
-9539
6832
4876
4867
202
-3484
6601
9890
4695
2072
5727
-7742
6271
1807
3488
-7984
8205
-4425
2470
-1653
8417
-2881
-4876
-3064
-6439
5261
3820
-1632
2181
-859
-791
3550
5033
9214
-1555
-6404
1898
4246
-4783
-8545
5882
-1595
-1727
5663
2971
1935
8431
1855
9027
-4882
1803
-9658
2278
-9730
-7099
581
-7524
5654
4655
811
-2153
4741
9315
-2098
-5804
318
-2150
7976
2428
8901
9902
3543
9466
-8720
3349
-8440
-7500
1407
-710
-3112
7140
1756
-879
3405
-1938
532
-6814
-3803
-1985
-2090
-5423
-8811
2935
1272
7173
-9335
-756
9888
-6963
-439
-8906
-565
8774
9459
7173
-9341
502
-8041
9823
-7728
758
-3171
-7883
-1832
4362
5540
-7320
6032
3757
3096
-3340
450
-7603
1766
1429
-3052
1736
-1821
6670
4495
-7333
4380
2182
4773
6223
-2088
9887
-6579
519
-8328
5381
3807
7448
4212
3598
-7542
3996
2780
-3141
2925
-3911
-8041
3115
8888
26
-3334
2412
4928
-3870
7706
-8494
-1096
-4320
8045
-1938
788
9889
2182
-4019
7097
2482
21
-6760
-7000
3008
-1722
-1674
2275
-2664
58
-8188
-1624
5688
-895
4945
6147
-4001
-9905
-2685
-3083
2428
8869
-5271
-5963
5851
-9197
-9776
9941
9739
7188
-9779
-9288
4532
377
3194
4936
-7928
-4577
7490
-3870
-7211
-6837
-9611
-8397
-8976
-9887
6019
7142
3405
3220
-4621
-2847
-6086

"
    .to_string();
}
