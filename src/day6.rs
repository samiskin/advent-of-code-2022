use std::collections::HashSet;

fn main() {
    let input = _get_input();

    // ----------- Parse Input -----------

    let parsed = input.trim();

    // ----------- Solve -----------

    let only_unique = |s: &str, end: usize, n: usize| -> bool {
        if n > end { return false }
        let mut set = HashSet::<char>::new();
        for c in s.chars().skip(end - n).take(n) {
            if set.contains(&c) {
                return false;
            }
            set.insert(c);
        }
        return true;
    };

    let p1 = parsed.char_indices().find(|(i, _)| { only_unique(parsed, *i, 4) }).unwrap().0;
    let p2 = parsed.char_indices().find(|(i, _)| { only_unique(parsed, *i, 14) }).unwrap().0;

    // ----------- Print -----------

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn _get_test_input() -> String {
    return "

mjqjpqmgbljsphdztnvjfqwrcgsmlb

"
    .to_string();
}

fn _get_input() -> String {
    return "

tnmmpfmfzmmnsmsjmjjbvvhnhzzfmmgpmgpgbgnnwffjhffzqqmzzbnbssrqqrnnhsnngsszsqzszhzfhzfzwzfzrrmhmghgwhhjjqwqttwhttjllrtrtzzcfzfgzznfznfzfnnbddvmvzmmfsmfsmfffhlfldlqqrnrznnhmmgqqzhhmjhmhppqbpbbngnlldvvdqvvrtrdrtrnttnppfllrbbrprpnpdplpmllhwwddqpdprddzzfccqpcqpcpcbbdhdjdjwjcwcctdcttzgzmmscmsmdmttwhwzhhnjhnhlhvhlvlglpgpmmjmgmrgrddmwddjfftfwflfslffqtfqttpftppflfmmhvhvcvbvhbhggpbgbppvdpvpvfppbwwsnnhphllbdbnbvbmvvzffvsffdldmlmtmccnlnbnjbnjnhhbfhhgzzlwlfflzffdccggdcgcjjhffjfgfgcczjccvwcvvqgvvqvllqzqmqllhjjqnqggttsdddjgdjgjzzrgrfrbrssrgrgdgrgbbssmdsdfddsndnsdnsdnnmqqsspqqmrqqpmmsjmmszzqvqrvrzznnjdndtntfnttgtctqtwwnwswrrthrttsdttlhlvvdzzgqgttnppjpljplpgpvgvqqvppzmmqggtjgtgstslltjltjjgcjcmjmsshvvtppgmmlslqqshqshsllbggfpgffdsdgssncchctcwwtllgqlqblqlqvvmsvmmwnnzppqllsttgmttftvfvjjrzzswzzjvzjzljjchcshcscbbrdbrbcrrnvvtctntvtvbvjvqjqggsrspsprrbgghdghhmwwldldzdttrvrnrfftqtftrrdsszlzvvbtbffftzzrzqrrhjhghhwbhhsjsfsttdjdjnjhjmjpmplplrrdjdcdjdbjblllbqlqdlqlpqptppdhhqmqfqhqhchwwqjqfjqfqhqshsmswsbbvssdspdpsdssstntltrrgnnmttgmmsjjrlrnlrnrnwrwfwlfltlzllcjcmjcjpjhphcpcwppmvmjjzbzvbbfnfcflfddntddbmmmhnnsrnrrvdvnvcvwvcwvwrrqwqccqmmswmmjrjmmwjmjfjhwrtbjzdvlgrjmvzfmhcqsncvlhzzncjlbvcwrdwjmqjcnptqslvfzpsvltgzsvjdsjrppdrmqrbqwhddfhnftfblspsrhtdtjwdnhbcbtlwlvccsfscvczzrrqmwbwbdmwgzqntvflppqvppwrhnvtlsbzqglhsfdgssqzdtjdpwrrhbnbtwhhnmnlwfwlqffjjrndbpwwsvdrhddbjnnqzmtpvvtwbcpndjzlhcfrrdvmljswjzvmfqcdsgqwclqshwrmblszdvsnrpdgnllmlchzdjlrrpndmmgddjqgjqrhwfbwddqdfbvptrmzhtsqfsfswpnvmtswqprjhbzvntgrlzthhnqbtpplqpvcfnpgdtbhqbhflltbbtmmhcwztslmpznttmssclhmnbsbrwlblrbsdfmnpqbwwmsncvzmpqwhzjgcgdrzvglgdtswmstdhrprdjfmqtjlmplbjtzcgnrwpdvpfjjfwjfnnpmdtwtqsgfndngsbmcwjtglqwtfrclbczfcmjtgcwszhzrbcphrhwmhcwghjznzthnwpljjltdlvqtffsrbmwcsvrdmqqggbznnlzbbqtgspqvnjpbdhtzmgttrcwwszwpgdrcnfqtgrgqdrctlzwtdwqppbhnwgldnqltznnfpbfqtgmmwpcqnndbgmrrtgtvnmlfcwsldchjnnqfrhpzwtclrzftsqllgvpqbgmfjdhqjttwcvbpvfqsvhbhhtwnqnbgndbtzhcvgglbhghbzrbrmdllmgfgttqmhtdnwrpwllhnghrjctrbzrcpnjnctvmrlpjhftnfbczrjrnnbqplplcrbngbhvmmvcffmgvbhjzbhcmtwmwgmjmwjvvlqfldswpntjnsjvmdlbzqqlgbwspwvmnwtwjbczmwplrhmjgsppnmtwmvsfwnsgddgwqcvpftcpzrhpldnwmcjgtjmljjbcmjcqdbwczndnjnjgrmtjrqnnjndzqdqpcgdqptdbrqftnwrgqmrzrvsfmmmbpltlncvtgrjfjmvtgwqphczwjhdrdwtfvgztbhrndvpcbgfjfvmrrljwrvcrtdmtjndfnwgcnfrzgsnjpztbwwsbvqfnpjctgrhsflhnzbbsfqbnmtnvrmjzsbjfndvttpvpfjhqntflgbfnzcclcwmhbsgqfjdcgsvrhtstspfzgvgglgddqmclsmzgzgtncdsfmwdvtcsgwvbzjvclwppqdjgfcrcbzcwbdhrnssjbmnmfmwthdrnmlfhqlddwqrdhsdvdcsmcgjsgcmpnhlbnqftpdjswtmpbznlcrhtswgnmwjcdfmljdngzfsmlzjjnzmfzshmztdbdmcqwmlvcrzgpmbjqcghclwvdbrhgvwqchnndftnrtptmctdlhmfjvpzrpccddfpcdwmzqfhnsqzrvwblzfhcjdcjfctczwqrcbjnrpdcbbnsgnlvqqmnsfgsqschjlbzhhsrbvdbfrhvsgrlzwncgwpdbvmblgzbwbcbgqfwmdmgcrbbjfcvmqgztqpptdhwmvmsdqwplpgcjzgqzdrftzhqbltvhrmlrfffcgfpqzwrrbbtlsjgmtbjvtnmhwdpjptjwfwgjgvbfqwmflrrqzlzdcmtlnptdrpcpdnswcfscnndnrfbgwvvncdjgsdpbwptdtvrqlmrhmvvcwblhhzbjdpsbszhrftfbcgwhwrgglnjzqdhcqnvlhgqjhnddvrslhntssptsbhmqwwqqnbvfmcbgpvgjbrttnvlljdbtfplgmbwtcbcdtqdpqqdvhbmpmtszwpzblcfrtznhhtcljtdlhjdbnlhvwgjsmgvrslrfwnmzwlstpgltvrgnpdqztvfnvdhdtwwqdfsmtpbpdclsbnwcgjzchjcsjmvhbjshmjjlpgdzcgbmmchwmcsddsvhsnpqtcpnhqnbvwgwqhtjbqncgwwftnrzsbsjtvqmjzqvvncmncwflcfpcjqgdtbsmjzzsdjfvhnqbgjhmfgjghwscthbfmbndltbqzwpqtmrswvprpmgwqnqpfnmffrpdlpfqmhrthppzvzwbrtjvwvjndsqdlqtbpqwfcttggnjmcqqnmjwfhfjgcvlnmtlgbdvmctzlwbfgnflwtsflgnfbnfbhhdgjctzvvmrhdsmvmmtnqwtszmqcpsbrqrgjfrzctcbzmtdlhwjtfdqbtthdnqcrpwrhcrvjstbhpltvgmvpmvfjstgzjsgzprzcqzqztvvdcnrrqwrhddcrhhncdrlwzwqlnbbzcfmqtnwgfdscmrbwnbldlfrqchzdnlnmwncgrzdclnvcvplgwjsbzmbnnsdrsfhrlssvncnwmcrjdjbjpdtrrvlnbjvspfqbwdpcnnpjzfnmbhcdhlmdgbpvbzmfltzstnznfctcdzhbfsvnfbsjqzmwfllhtrsfghlrpjgrgzgchlwrdmqzbrncsvnwhfqmwjbnvjctzphcsftqsbmwntgvjqhhvwndvmfmjhhhmfdvrlhpvzmmhrbhbddqbdmgqqsvddsswmzqcjmvhztfqpchzpwhdshzjlmbmnsgzqhbnmrshwvtmgmgndtddpfwsjrrjdhncdhtlczdvlbvqplttnzrblthlcffdtfsdtpwzdgbldvnsttvpzmbgnqddrszftcpwrgmfzhjjvghpntmzcttcsnrjnfpqzqqqljhzlrpgwngllqjwnwfcsphqplgbzmfqfgbfsqpsrntszqbcqnhctsnbfshmlbwfflrwwsjwqwfqlgnftdwmctmclwjhjhbsspqldlshbmpbgrftpnbpsqldhrrbdqwfwvfhclrlfdjfmzgmptdjdcsplcspznfjrfhtsjndwpslrdgnllllwqjgznrhswfssdlvdpmwwgmstqbhfmdhtzvzzvhwzbrrvvsl

"
    .to_string();
}
