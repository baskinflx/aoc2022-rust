pub fn first_problem() {
    let lines: Vec<_> = get_data().split("\n").collect();

    for line in lines {
        println!("{:?}",get_marker_char(line,4));
    }

}

pub fn second_problem() {
    let lines: Vec<_> = get_data().split("\n").collect();

    for line in lines {
        println!("{:?}",get_marker_char(line,14));
    }
}

fn get_marker_char(line: &str, marker_cnt: usize) -> usize {
    let mut dequeue:Vec<char> = Vec::new();
    for i in 0..line.len() {
        let x = line.chars().nth(i).unwrap();
        if dequeue.len()< marker_cnt {
            if dequeue.contains(&x){
                loop {
                    if dequeue.remove(0).eq(&x) {
                        break;
                    }
                }
            }
            dequeue.push(x);
            continue;
        }
        return i;
    }
    return 0;
}









pub fn get_data() -> &'static str {
    return "mnlnvlljqqccznnjtjljbllrtllwwpmmhjjbbzppnndmmsppdqqwvvstvssgmsggmlmttnvvfbbdsssnzzbssjrsjjpmpvmmcjjwsssndsslwsswtwnwrrslshhvzzsppffmpfmmfvfpfpsssqpqzpqqcjcjnjcnnzbzjzpzbpbnbwbcctvvhgvgsvvpwwvjjvqjjjdqqrmrmqmsqszqsqpsqslsddhbhcbhbchcvvjvjcjnccdbcdcrddldblbffhvffpvpzpvvmvfmmwhwqhqvhqhmmpdmmlbmbgmbbrqqpmqqcvcmvvcncllptltvtdtbbqzzcggjgsjjvjsvvgmgffqhqqgpptspsffvdvbbhqhzzllvvjbvvbpppggfpgptgtvvzdvdzdgzgccmmphpmhppldlnlpnnhghhrrgwrwssnllmpllbvbvqvtvhtvvmnvvpgvgfvggtztpthhcfhfqfhhnhtnhhljhjppqjjffgjggrwrjjhphzhtztggwswnwzzvbzzmmtrtqtjqttwlwmmmmnddmnddwvvcllgrgfgzznwnsswjjhwwspsbbvzzqvzvbvcvmmtltnlnfnfnwwsvwwpswppjhjdhhmbbblfbfwffwvvjgghwhzhjzzrttwhwjhjchjhggdrgdgmmsjsfstftvtmtctggcwgcgzccgzgsgrgmmjhhqzzrmrttgtgfgcffvsfvvslsvvpqvvnjnrrwdwcwcnnhllwpwdpwdpdqqtwtftdftddppncpnccllqqrffpssgvsvtvmvssrbrhbbzggtssdsvddqfdfjfhjhdjhhncnddfpdfdmmrddncnvcnvccgvvhzvzwztwzwtzwtwqttrlrvrddztzrzcccgmmqgqjgqjgjqqspqqpjppbggchcqcpqqgbgdbbspbbrbhrhzhqhrqhhhtbhhvshspsvsggjdjwjwvjvdvwddjggmrrbnrrztthlllhlclbclbbhpbhphjhccdwccdbbjrbjjmrjrhhnlnjllltwltlmmlqlnqntqnntsnsqqvtvwvgwvwnncgcdctddnttfjfqfttrhrjhjqqcnnsmnmgmqgmgbblcctntrntnccnvvmpmjjvfjfrrbpbttsbttvnttmnnjdnndnzntnrnwrwcctllvhvqhhddmzztppphghphzzglzlnnfccrfffvvhllpspwssstwstsvttcrtccfssbccdjdqdfqddrbrqbbtllmmsfmfcmmzwzpznpnttjgtgbbdtdvdwwpmphpprsrjrbrqbqwwljlslrlrhhpchpcprcrtcrcfcfssndsspddcjjjfmfqfggmssnhsnhnpncnfnmffdrdjdhjjrgjrjgjqgjqqlmlljffbcfcrrrzwwftwtrrpgpprqqmrmhmwhwmmcrrhqrqwqppwjwggpdpgpvgvzvttqlljhhbvbhblhhcsssvmsmppcvpvrvzvbvtbtssplpgptgtnthhvwhvwwfvvfwwmtwmwfmfgmgnmnllgsgmsgswwhqhhhzqhhfwwnttmfmrrfnnpbpssvbsshqhqvhqqbmbpmbmqqjtjqjvjtjjhtttpzphpqqwqfqttqqhfhbfhfwwcpcpssdvvzhzwwqddjzdjdldlggvnvlnlbljlqjjmcczbccznnlnslswlwplpttvrvllfwflftllhclldhlhddbvvpvzpvpmmrccvgvdvqqjcqqwvvnjjlbbjwjrjhhlzhlhttljlcjjsnsgngrnntzzbsbmbsbrbdrdppjrjlrrjljqlqhqqnqsgdvhpgdhmnslqtjclmcfzrmgmlfnjbzznfgfprvwprwdbcgfcclmspgnzpbshwjbqvhzhrhswjzbfvnmcjtfvqbwmjpvfvctpmwsspdbtvfhfdfzjdpqnvslgmdvrnflzwzcnzmvzsvznwhpwtjwnqdgrrttmmdwzbbnwtllpbffrgtpjjjwltqrcbqcttdwnfjpmhdsbbpqmstjqchgjvfrmrbgqrlstnbdnzzzbzbsmsnnsssswmqhcbswtjhmcgnwmcclhzjqjzqcpbzgdzjgqzpqbmvvhtcznfrhdndswfvfhtfpdpszpjqrlwfdscvcngftwqmfttjtjrlbgcwvcjwsstqmcblmjzsgtgrqnqqvhzhvsphjmbcpfcznlcqldcvhlsvggbjngmhspwwqhlwstslvwmmbwqdmrgdvvnlstmjllhzscrhzjtmnsjfbndnlmzqbzgdgbcqchnbvwsftjtznnbsnvsgzpdzdqznjsslrlfnccdhwsljhczggvmgqswjltmrqqmwtbzmtdzhpjcvmwsscsdzpfnwlcrrdgzqqdmgwdlzvvvjcqsgpcwvrdnrstpcmgfjnjffbfmgzjthhllzrlsjtnqfppltbrlnqnjvqlvtpqvsbfgmmlcdzhgmzzqjwtqtzmpwwddbqrqnfzzpsjglsjddsslwwlrttzfzplmwsswlnvrvwwcgddjwcmvsjjbfgcfjmthfbpmcwjptchhnsmzttjqnwzdljffghhqdcwzwgbvfsmwqdbtblphdgcmbhprtbccjbzqrpvjdbnsmlwfntvjgptnshzmddwbhgwsnfrjbpqqwlsfdpnmmnnwhdmhzvjcmddbdnjzfzvffbgdqgwbggprcrbzwhvtzzgbhhcscrlmfgztfswjbsnwsmdfwlntwjzvlwhvlrfzszllmflmrsrcfnncvszvgdmmnvgrqnjhljcnrrhpdhffwmrsqfnbcpfdmmgppwjbjrwdfmpcrbznrjnbmssszhnlbpgmlczhhcdtgjqcbqrvzcbpgrftfhzdqthhspwnqqswntlpcmmqtcszngpggqvfjnmnprhdfjsngwrncjcmqdmjhpdlfnshpdlnlfpcnprwjgdvwwbvhvsbrfjtqsqjnvcpfdsrnfwmrrbtcvcqzflhdlbpcthzthdjzsrvwgbhjvhbtrngthfrszlvrbtnscsqlblcwlngslspcrhqzzdlzcdbhqdhthlpmdrntbhnqtwtzwpndbgphpsllbvgqjtmszdvjpgttzcmbwgrgdwmsbfgvgbbcmsnhvmnsbcsthsdwdqtghpdclfbglbdjgnnnwhmzzvnhbmgfbmvqwvwqhdswgtzslspmbmznnwdmjbzbddhzchtdzdzgwtlmlpmwrqvghpfwhvfjrtvmjwgjjnwdwnpdcqjdmcctjfcrdgpvczvnhlrbfmqgnrhmdwsrmmpqhvwgqgbqccpznpjfldwpntnvzgdfzljmtqwvfnrdsjsqgbvzjsczwwjggqtrpvwgqggwwhqggtgqfjmzsmjvdhdwqggbgnftpqqqlsfpflwrdpjwnhfdpchcgntjshgtwnwrnpsvwmplvqcltbgrcpflpgzbqfclghfnwjchnbgjnplgldmphdplvjrnrtzcmlftprsnmrjmnffpqjlvqlztbwprjwprrmmgzhjgdnhbfdrwjtvsvnbqhtfhbqgdrvcwlwfdbbcthgvttpvrwrqmpmrmvgpjzwlpvbqcvgccpgfddjbwhrvgqmjqzwgghllrtrblcpbttmcrgjsftlqhjfvqnbhmhbhngwnfqtgdttstzvmstrqcpfjrdgtsdbqqccqvbhpwhnpmpqgntfqszndjrmfhlqjqjbqvjtlfmrnnzzrtqlzzhjfqmmsmzvzrcplmfjpcmpfmpzbsbrmbnbnjqwjcfwnnwwrwzvvrsvhvrwnhlmwqjdztqthcwnwrlbjdflfsbplbwfmzqqnpwvzjbcfdgztpwttlrvhlfzzsfltpqwcpnzlsqgvwnqfvgclrfvssfcfmfvvjsndrhqdbrqfggfhjbvdvvmgpglqzwgjdmqtscjpfhgsbshghtmftrrhznttpzrzcsmrrvzdjmwtmbcbpqbsdmqzqdzrncwzmptltvdphsltfrhbbrdzbnbsqdhfvrgvmbgfvwblsjvfphlpzfvsllwnqjmbhngzzslcdmdzfgrgscbzggrzmbmwlzbnpzcvsbsfgdpnwzljsf";
}