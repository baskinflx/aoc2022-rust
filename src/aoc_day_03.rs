pub fn first_problem(){
    let  lines: Vec<_>  = get_input().split("\n").collect();
    let mut sum:u32=0;

    for line in lines  {
        let input = line;
        if input.trim().len() == 0 {
            continue;
        }

        let split_line = line.split_at(input.len()/2);
        let matching_char = find_first_matching_char(split_line.0, split_line.1);
        sum+= get_value_for_char(matching_char);

    }
    println!("Problem 1 - Sum  =  {}", sum);
}

pub fn second_problem() {
    let  lines: Vec<_>  = get_input().split("\n").collect();
    let mut sum:u32=0;
    let mut triple:(&str,&str,&str)=("","","");

    let mut cnt =0;
    for line in lines {
        let input = line;
        if input.trim().len() == 0 {
            continue;
        }

        if cnt==0 {
            triple.0 = line;
        }else if cnt==1 {
            triple.1 = line;
        }else if cnt==2 {
            triple.2 = line;
            cnt = -1;
            sum+= get_value_for_char(find_matching_chars(triple));
        }
        cnt+=1;
    }
    println!("Problem 2 - sum = {}", sum);
}

fn find_matching_chars(triple: (&str, &str, &str)) ->char{

    let mut matching_chars = String::from("");
    for c  in triple.0.chars(){
        if triple.1.contains(c){
            matching_chars.push( c);
        }
    }
    return  find_first_matching_char(&matching_chars[..], triple.2);

}

fn get_value_for_char(c:char) ->u32{

    let is_upper_case = c.is_uppercase();

    let mut val = c as u32 - 64 ;
    if is_upper_case {
        val+=26;
    }else{
        val-=32
    }
    if c == '?' {
        val = 0;
    }
    return val;
}

fn find_first_matching_char(input1:&str, input2:&str) -> char {
    for c  in  input1.chars(){
        if input2.contains(c){
            return c;
        }
    }
    return '?';
}

fn get_input() -> &'static str {
    return "DsPhSBQQQhqmBDhPDsFwjwsLjlRjlttvjvvtRb
rNJMNNbrHrtjHLHjvwtg
fNbNzZdrZnMnMPnQShFPDmnqFm
QWVCFfQffgQCVZzVVpHsHJBqtpspJFRHqq
mwDbmnnGNlNcwNDDNRbnNDlJTpBJBtJGtPTLsBGqTqqsqp
MlSdnScRnnmmDjSdNSdCzvggWzrgzjvfvrgVzW
gsMljbrjlZlWcWMJrWwTwbmwQbmmLDQQLhwL
CdgpzdgpgnfThHfFRwhfRf
SptgpSpnCNpVSGNPvPGSddcMWjMrjqBsJcWqMcBWcVlZ
JcJLQQFWhQJPJpWcwjHvMQvnnlMvzBHd
tCtGZrmVRmVGTVTtCfRTCHHNNvdNzmdMvMlNzvwdvw
CTGGRftfSGtGTGDLbFchSgSWWWcM
QcMFQrvrQbvtczbVbjbMzZzRpqmDDmqqnNzCDCDC
SHHfPJssGLPSdHThLhHdRmqNmNssnNmNCNnpjmsn
LhLJfTdLJwfgPTdfGccrtjcMrccwvcrrFr
jFLLLqDGjbtqLCChpMMhMBvpwMTmffpZ
ZnJHRncHHgnrsrZffTdMdMBfmMvfvR
NWWPnZrVHrZPCDDQtzDCPLCq
jpFjvBZhDFHZdwcmslcslBLLNl
dVtTVVCzzfrrMPNLLcnVcPLRns
CrzWzTqdWtGCzJtbJCrMjjDFHZQjZSpvFGFgHhHp
JjJqMctnhtDZDQtf
TrFFlrrCCHPwHwlPHFPzDhsffQQDsVfWwVJQJB
HHHGdFlHldTpCCFFlLHdCRJccMnqvqMgnNjjMjjvLMLcSv
cMfFcMFcrqgJLFZdrTDdthPGsGmtGs
WwjNwnjjBQzVVQHwRDnmtPGhPPsPnnTGld
bWHBjWVzpbRzbmScqgZMFcqf
RJjPTBJhTNNjfPhRBdqtlgdbGldwtCPgdb
ZmrHHmzpvSvvpzvmvDVCGlWVwCWtGtWCddggqf
QQpzFrHHQnzHvfTcNshcLRNFJhcR
QrPQDrppBQmCmFQm
TzqzsLfmsfSTfqzVLftNdJJNJGCwwGdgCwSGNC
TVHTfzWsfftsZstnbvrbWbrbppPbrmmP
BQrfqrLtJnttqqtQBJDDtBnDzRgldhVVpJlgzpzhpzsgslhV
TTNcPZCvZjmPFZjvPHLlWdNLghVhzzlllpVd
jcFPbTcZTFcmcjMjjFjbLwbnDBtfqbtBwfwSBnrbGw
SZJNJtrNzjjNCzlBBmqmQDBBmDBBjB
PGPbGwhLsnvwnVbGPVMZsDsTBBgcZgBBDBfDBf
pnnhhvphGVpvRPnJlSFRNNJHZHSdSR
hbmDDmwnnVGbhmjNrrWwLNLsWBrw
dMrgvcQqdjlBLlfW
HzzPSrcHqFHQgzpPcMpQqrtmbbbRhHHVGnZDnVHhVbVZ
RwtvdPRvSlTQmHHBQBRL
FjVSjrsFVnFQnTHmnT
VCVVMrWWjVGgbcNVGCCVdlvfdqfvwbSzwqfwZzwv
fmPDwJPDFRmRgPdwwwDNwgwPzhSQzVSzVTQdzHZzHhzQMQzQ
sWtGCWtpcqqpNnQjjzhVTTVQczhj
tWBGntsCNlqrWswDRfFrbFrPDbPf
dmzmjcbQjjQztFNqsqBcMJqNPF
nlTWnClWwQDTVnTrsMsCRBPZRJRJqRJq
wTrhwpVVvgvlDpTvVWVDdLfzgbLtbzSQLSzzjzLz
RWZdHvRdBRGbbvCjJnbn
pqqpSwzpSSbCwPwjlwjl
gDzzqSVzqrThpDBtRtCZdQRQLrQQ
GRqTGqtmTVdGHHVVNNlhPlMqbNqNDbqW
LzQSdFnfznfwBcLcnFppBBDDMPMPhDbhlwbWPlbWMjrD
QznFnLZFvVvHRgsddG
BBHBfBHFdSltmWJvqtNtHq
TQDrrVMzVDnWnNZDJc
gWzprTCrCMQzGGjjhwFwdffjfl
LLLSSSzBBlBLsszncCBZSPSMMMpdWTdrmmnfVGVmMfmrmm
DghttvwhHRvjJthNJwhVtDHGpddMWdMmrMmfGdppjWTfpq
gbJRthQDDRvwtDhQhDFPcVcbPczLzPBScBBF
sZsZpTtLCsbspZtTwrCwrwtDmNNdJmmqSJfqmfNGNmHqGdqb
nQvjTcjQglFVlllMFMVFqWfNqnNHHHdfSfHqfSJq
hFvvgFcFVzphpTrrwZDp
qJqfhsBpfSpchpqcrqwCDvvCDQndmpwDtnRQ
jWHPZWWjZsCmCWRDmQ
PPzMGHlzMNsPLPlZsllgsNrhNVBqhFVBbBSqFFBFrV
RnRsFFdSzmgwvQsqwc
lbgHGMBHlWWWlfWGGBtGwqDpDwmcpMrrqrQDZrMZ
GGJLBVJbVGjGtHVfJtWGHBLLPdzdCghnRnRnSShgNnhnddhj
VgzLFjjwhhSwFhVZgRhRgHHCCvdZdrqqCTvBCrqvvr
ncnNbGMcPpvHFrssdBHM
cnctcNNbQGWFJctftgVmzShzzwwVwgmwfR
pFWmSSFGQlvTbwWTwH
jdBgNhRgMftNBhPbhHnZHlTTZcwZ
lfBNRjCtCfMjsBfCjgfNBRMppGpJqsJQGrrmmJDpGmDDFG
GznngnhzccVdgjbbVjVjVbVLwwQJmQMrLTZhJmZLQTJWmm
pDBSslPCFPCpvCqvpPBQTlQLrdlJWrLJLmrlrr
psSsBptpCBdjtcjNVcGG
sssppsmchwspFLtvHhQJMtFb
rRLDqRVLNSMvFSSV
WDrLnzrDqzRqRzzfLgRnzrnsssZZZsZBgCwwmBppwBspZc
MtPbwvzzVtzfsqGGVpdSjsLd
DnNRCDJBnHJDHDnrDTRcnNZpZSsSLqjpdqqqSMqdddTj
CFRFRFHgDRnRgMtfFvvwzwvwvzWv
PNpFPncvvchPpNjpFhvPhPLmBwMgDRRwRgMDhBmLzBDD
trHSrdTtslWrSWmfzgmMnfBzmn
JsrQsJHHlsVqTjjpbbNnQNNpGG
sVQCdsmGlnlCmnGmQQhGCJJNvNjpgqhqhvPgpgPqjpcpcW
DMSfSbHLHbSDBBzLNvccWRcddvPjNj
FSwHfTFdFsmJlnTTmV
FctwtTTCScvShFqtwScrcTSCJQGNndGHWJNQHWHZdgJrJgGN
jspLlfPlpfsDjBspfllWgdWHQggZHngHNGdsWJ
lRMpBlPmSFRTcvZv
TVZpRRVvFRVpTZRfFhFvvzGVwrwwwdDBMwQrgcDtMtDDwZrL
jsNsWqWjNQCNWbjPMcBLPgBtrdMwdrdL
SJSsmqlSNljbmlNjsbQVffzGRvzmmVhQpVGz
BFFMvcwMwwpFFfpbDMqPVgLVgmLDPR
JtSsSzJssQJJWjRZzgLnDgqLPZgzbg
dsdtWQWdRGjTRNQNQvGfHCCFpvfGwlCfCp
zzWGqWnqnwWCvCrHffHRpBpBBRSJzRFFDhSQFR
VPVsZMNTLsMvPsmBJhllFpFBTJFQpl
sZtsNsZtdZgjbwggjnbvqr
QpTvrphmDvvddfcJJHTTncMlMG
bRZZPRwjgzzlSSjGlnlSJS
BZzBzssWgwzzwNBsgsPBgszmQptqrrGdvdWmqdhhFQDpQF
FfMtzSqlDlzfMhPFhPtffNRsCgSgCCGspRpRGSsgsg
WTcWLTTVnWmrVdLrcHmNGNCZwCCggpgDRwpCnR
JcdWJHTJWdJdjJccrLvlFltFQQvPFDMjqqlP
NfjFNNZPDQVJVWpCbQpJ
lcmdzlmzBtRSTlTTcncsVSbhpLWpWgsWghgsVS
mBTRRmGccRtBwDbrGPbrMMrF
VVQqlsGrVsMWBNFNMQHF
TfzZfDgjgnLGjjztTncCFwwNmdvNcwwNBvHNFwvd
jzjDgZzjntGJZzCnhrrSlPVsJslpslPq
dVhpjGPdjHhqHgtHJJ
sFzrzllQswDwFbcmBlgvZTCgvqTCfgHQJtqT
nDbBsFzzrrtFrlwzPdSVMWMVVMSnGNLd
wqJCjqChmwMLmMmprNgG
DNTtdsdWcHdNspGQggnrgLnQpc
SZvFftdddDsDTtttTDJNCCwJJZzbbCjwwVPj
VstwZCwslBZQDBjfDDBDfS
rvHnmMRrTzmMrmhRppbhDfpjfbjbctjD
rPFLRLmFvvLvHvTCtqsVVwldGZGPGV
SNZDJGfvwgMgfgmLmLcmBqgWgQWr
hPnPTnVGsPRqLWpTmQqQ
tsbnFhhjhSdGJNCjSf
QSbGgBjfTCMWFNPFFtDghPrP
JqHqJVzZzwJJHLlqQptDhPrctpPDtnLF
zZmwJwdwdvHddVJvZqdzHlfGBbsCmMTsWCBCjsGGBbMQ
gVmLtpWrFTFBLtpcFNbvhNNnTnvnQRlQQv
GHjjqsMqwZZJdqGfZjfZGjQDDhvQNzznzRzRwDNbvgzv
sZGPgssMqHZpPptmCcpCrP
CZCNNLmwzwCGMZQMQsFNWplvpsJBWpFHBp
SPbSbDRRbnDqggnbVbVrbRWHJFTlBBcTvWvsPvFpHTcl
tVgSvqVbnqDdgQzMQMGMmzthCw
mpbPQlblbwSlfSPGBpBGPpBFgMMtLFSHMLVVDVdtHLNctF
WnhZsrJhTnWrgZdZgHNLcLtt
hjCsCWRhjzhTjnWnQBbmPQQPLBGfqjwQ
BFrzdtmRmpFtFwwmjjzNQllPshqHvjNh
fWCLLCMJnCDbgfMJhPsPHvqvqVsssQ
WbLnCZnvprpmZrGd
gjMzTGBjWFBCCSSrBC
bddJJndbdJfwPPthrrSSSsFSSg
NJHbZbbvvWGgjWgT
dpfphMggHdQcwftMMgdtzWGfGWnDBnmvnVJVvfmn
CqPFTZPSNCTsZZZRLzGmcGVzDLGvLWBWJm
TCrTNSScPlplpHrQrQ
lqrCvhWFvMGWgfHPgLfjfdgG
zjmbjSnzRzVVRmzBRtwjVQnNLfdPgfLdfTtNLPHTNNpppf
mbSZRzQRBnnzbQJbmjmSbmVhlslZMrclWFrrqWCWsFhchF
mpfNshshflNthWfJCBBdmnQbQBZQdn
VFVRccgGTqTrHTbWBFjJBCFFJCCQ
PvTDHqHqPPGVqqhwfNlWlDhMltlh
HmLLgWVjJwhwWLgjjhmVHLLLlSzBlBlSvBvBFGvtdStJSSvq
TRRrPMsfQTbRRCZRnTMRZZTCcvSccqBqBBlzdFvBqFsGBcts
bRNPbRCZMCrQNfPLdhVpLDVgDhNHWV
sPJFDsSsVLgHjLHPbj
vCnRQhhRQdVQZlZdbHNMlqNNjgjbpbBL
ChhCCCWTWnnmvmGtztStzScwrcVcFW
NRBTNDBglSSgDwCClQQSFFHdLLsFbPFFLt
McpmWccMWHZPcLstbt
MWnWphVMvvzJzpWJWmVphjrDDBBTRwRDDDRRCnPDnqgg
hsnnhhLljLPTmZwvdZdZjmmz
RQNNDpNMSZwvsmqstN
QFMFRDVDsHSpRpHSMRHfGGGTLhCChBGhBhBFBJ
TTbltCvClzvzCZtwtwLTtQQQgjNgmjgQRRRQSjQLjR
HPpnZHcJsdnnfsdVHHSSqghhmqBSSqSmBfNj
VMJJdnMPrMGVrzvCZwZWCzCt
CcQnBBCfBvRzDlsS
PdbPPCbhGGpDSDlDDhvDJl
HmHWPWdbHnCwCHCr
JHlmJcMWHQcPmlmJMmMZPfwTTRDfgdDBfRtgQgdfBw
zVvFrqGjzWGVrqvjvNFpspvVBgNNddtTgghhRTwgdDwwwBfT
bVFrrvsvjWGVsCsqpSHZmMPJlJnCcLcPZZ
QrrQZFZnRtnFRTrnlFTtRZwpGGwzGszhjzbsGzzhmjjhhmhf
SgvpDSJSgPBSDPDNgpggmsMhMmffsMmzfJmzHHMj
WNVcCPgSSDCddSdWCpNBclCQQFRQQlnTLFLRZQnLtt
tgvqWqzvntdlzNzzHrbPrMhHMhhrVrjPmC
FGTGGRGTffcJRpQcJJwmjhPLhjnCQPrLPVQrMr
SppRSDRwFnJJwwswTDdtzBzqWWzZDldNgtvl
sjzPjLphMSrlppSp
fbGqgwTqgVTFFcgGTTGcQvfHQZRlSrSlSmMNMBRNNNrZmm
MVcFGTFtqjsJLjjtCd
jVJqTRHjjtDjZnVJVVjnNdwfCFPZmPgCCgCZGCFFrrFg
LSLBMBWsbBBCgfdrFd
ShbbzdzQRNNNVhDN
JwbSRqmSwRwcpmbSSVfhNBTVGGRnNNzRNh
ZPfQPjZZrffQZszhzTnVGBvntsNz
rLjggjQgQgFdPQjLFQZgwLMbWccLSwWHqHfSqpwc
cBhzNpztzHNrpHSHQrhZjZqdJRRgPqjhjqqP
bQwsMTswwVjdZMVRJM
vGWCTDCTQvvtlvQQ
mHVmTTsmzRmRHffmmfMGBzSPMhSWWdDWDSGB
VNcnNbtqqJbcbvgvWhShDBDPhjgPhSjB
FrrbCnqJcbCJcFFbnqJlppCTfQmfmZQTLVswRLZp
rWWppSStfRBLfHPdHHDzlldZ
QrTnQmmVCqvrvJvzJZPHvzHsDs
qgMMmrTTCcnQQmCNQNQmgTpwRWRtwtpcBWwjSRSSpttW
SzWQwwqVbQzwGSfVwffVwbqhFGFFMgGLGMlNMcnNgcvlMg
ddZHHsrCdBssDsHmChFLNNnMvmNnnnFncL
rpJJdHZJdZHQSRJbSfVhww
jLtFhNwNNvHnjFVvQLQLHhfbGDMPfffpllpttZMPPZZb
VmrqszVmgCWWggbPGclPlZbcZCbf
JdrTmVTVsqsRJHvQnnhwhBLH
vpdBvsvdVvSPhvQFCgrRmhjqqRgWcWWgmf
tJDGbZHbjRHNrrBj
DnMnzGJJwtzVSzzBzTBSTF
hZMDbQhMhlMDJrDrPMJRRqGJSvFvSwFSVdvGvwSFqq
HjjzcRTnpGVSGGFWHH
pssmjctLzpsRRnhlQbPfPrgDbZls
FJMhPhnTMGmBFdnmlrfqfCcwlwCwCv
pjHLsLbjjRHsHsjDjVsgDRjzCnfCfzqrqvwgflcvcvvgqz
njtpHZnLLQGhTtTWSSSh
NMddMTDrrjNnrnMWMrlnPwNwftBtGvptGjBHgBgpfCCSfgvf
mqRqLFZRVFbtgBSCbGWWHp
LZZRmQhVLLFzZdTQPndPNTTnQW
CprQrcpqhHhpppchpphdcdpLMRMGsVGGsMsLbZbtbfMRmtbG
DzlgTjwPTlSSSCDSlggNmMRtbsstsMfbwtVssMmt
TNCNSBNJJTNPJjCTBDzjlJrvvcQrrhphWBqHhHdqrnrr
ScbcbglMPRSmvclTlzMTdhVHhhjfdsdhrVDzfjrD
LJGtpQwpQBBCttJLGqtqGHhPfDdHwjjhrVrhdrdshr
WCpZQCGqtqQWtQWplZPcvMmgmvPccbvT
tGWqthqGVdzBWwdd
BZvZHFFHZrHZvZNRRHHZNHrMMTzRMTdTzbgDVMVssTDTwg
JnNmrHHNrCGBPPnGGc
wvFvZBmppBdSLcvshcLrgl
bTQqJJHJzJjDQjQtzRVJztDHslgNNVLcgSrsSsWrSslWgrgL
TqHHjQnbHTbbmZBBcFMpmBwn
ZFCqSlCPdCRTLWWTQQLLQS
GjggHcvsggHVTVBLnlcpBQ
hvGhlrtjmjrHHhjsMPFNPdtqMPqqRfCf
cgRwVfVzrgqqwZctTztFMFMTCdPmmF
bWJsSJHpTDRDTWMC
sHHNbshnlJhJjBsjsbnBHbZcqrZZQwqvvZcrwwvRcZnr
ttgVBtMbttngmHJVpzlzZlppzw
cPcsRPmPPQfGcccRQSSZzzvzlflzZHvJppzJzT
sQPSCPPWccWWWqsQqRCQFNRMmnbnjjNbgDdBjhBrdbdDbd
gSsZDSgdQZgWSgddwLDLLfCDBqvlhBlClqtqjhvBhB
RMcsVMcsbVsrPrmJcPHHVvfjjCnvvfCqhlqvqqqq
NFMRRpHPgzszGppG
WmmZZNJCgCBZCzPvmBCmjWjjqFtFqjFjbVwqwjtt
ppDMpdMfSfSSNtlwqbqwwq
dTGDhdnfLMLhfDDpQvZNZgmrmrQJgrLQ
jrqRqfNNhrVvcTVdpTscpd
tGWBlLggWGddTspZZd
JJzWtQgQsDbBgDWsgbFrPrRJNMJMPqMNfHjH
GdvVbbWsWdvWGDvWZcbFBTBZCzCjwzrhPBJjzF
nQPfSHnglfpnMSngnMgJTprJCCwwrwCwpTpCFj
SQQHSttRqggmlVsNcvWdRvbWPN
WLNLCWLsSJgHFrSHrM
gDbzRTcmbzPPgPPdtBctfHfrMMQfGFJrhQnMfGJR
dmmzzqZdDqqTLLwjgZgNljws
TCqsDHssmDsDGVGlVcMccGDV
FLntWWnnFzFgzzbtbznLBtlfjGZmcfFGVNFGlVZcMNlN
PpbLLmpgbRzqpCHwChQqwh
lVBPVgflgBVVrVTTwgwBPmgflDQjnnGvqjGDDFjGlGnvqHHJ
ScCLzdCZdbLSbFdFqvDDdGMQGQ
bchNCLRWSbzCbNRzbWRmsrhQwVVwBhmQpPwpgV
GmfHCCPqWqHLWLCfRJpbrrbQphwZbZ
STVDQzNnzMzNdstMDzzNtZwFhrlwVJJrJlFFJRFwFR
DzSvzstnszMnjQcqggjPqQBcqq
BzNwLTqwTjrBrljWpBlrQpBpsCGGCtpssCMttDtHsttMDCHp
mnrJVmRhhvJVcHtsGHMCnZCPZM
bvSmVbRhbgmmSJflzNLrzqzfWfqBWr
qsZLqqFNhfrGnJFv
DVjVmWlcjVjdDTDlbbTDTzmrCnMmJGnHnmCHfJGfvnfM
lVRdDglTlTgVlVjbWWVzVjvNZqSBsLZNtqRsLBqLqQtL
ffRRfLVHZHfJHVJcMrMZZwwTzGBBzBTBjGTwrwdSCj
qmWbWQbqnbvWhbQPGnGNnpvwSzltpBldtSdzllptCSjTTt
vWsbnbWnNvQQQQvsRRLfJRRVGfDcfV
PpqrvswPvvvPsNqmDLDttCCcHHZVrCtW
hgSddBhghQMdbhbwhgSdQgCCctZcDtLntLttCVLtSLCc
dGzdQMFzQMdgGGlGJMzBgBwRqNNfmmTsvfRfJTpwjmpP
pcphpdrWDmTgHWMtRWqHRVVH
SbGcGGQnvNQGvsjBtVHBLjvqVPVB
lzbQCzbZllSzQslpCJpJJDggcggfDT
ffrTlhrVrfCsDVTsWzGBBZcBRBqRBWtGtZcq
QNFmFHrHNSSmSLJdJrNGZLbGjbcBjBGjqcbgjc
ddQvPnHddnQJHSHSFPdnFfDDTfDMMCrwMlPTwDCsrC
NBnrNHQHBscvhfBM
bbGtgWWWqZFmnZGZbmVCdMzvvShfzfGcdcfMSS
WqbjnmbnjWgmFFgtWVbWZwNTNLjNprNwQLDwrTrHQr
lwwlcjlzQRBcBccbdLCVnNVWJJzfLndd
pZDMtZpSDMZpMZMsPtHVNJVfRCRddVndnJLSJd
mDTZvTqtDDHttRPcqhQlwgGlghbqBh
DTspTqssqTbdmCMwVmnmlfCD
PLPWjjrSjvPzqZvjvRqjLtPwnnHCmWVcncMnnwmncHcHll
RFztFjjqjQFQTbdTbQ
hTFSVSdhMMVMFjjgbthcbzczcg
DvwCJVJWWJDRnfmbDmccfmmgfb
QrJvPGWWnpVMqdZsZpqM
gRLcHbgnfpgpJjlqqp
SFwrTHFBqlZtBPZq
HTzDwmSvCvCmsmmvTSwvFwcfRRNRLcQRWNssNNbRWLWL
PpMgDMDnsWSnjBQnrjbn
LFcVVGChCFdhdVFZVpVCdVbvQbRrbvBBbBjQSJZrrJrR
FNwGNCCFHcVTHcFdHHHTDzMzfsgzwpPWMmPflmtt";
}
