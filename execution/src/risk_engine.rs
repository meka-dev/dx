use std::collections::HashSet;

use cosmwasm_std::{Addr, Uint128};
use terrars::dex::Pool;

pub struct RiskEngine<M>
where
    M: RiskClassifier + ?Sized,
{
    classifiers: Vec<Box<M>>,
}

impl<M> RiskEngine<M>
where
    M: RiskClassifier + ?Sized,
{
    pub fn new(classifiers: Vec<Box<M>>) -> RiskEngine<M> {
        RiskEngine { classifiers }
    }

    pub fn exclude(&self, pool: &Pool) -> bool {
        self.classifiers.iter().any(|cls| cls.exclude(pool))
    }
}

pub trait RiskClassifier: Send + Sync {
    fn exclude(&self, item: &Pool) -> bool;
}

#[derive(Default)]
pub struct EmptyPoolDeny {}

impl RiskClassifier for EmptyPoolDeny {
    fn exclude(&self, item: &Pool) -> bool {
        item.total_share == Uint128::zero()
            || item.assets[0].amount == Uint128::zero()
            || item.assets[1].amount == Uint128::zero()
    }
}

#[derive(Default)]
pub struct StaticPoolDenyList(pub HashSet<Addr>);

impl StaticPoolDenyList {
    pub fn new() -> Self {
        let h: HashSet<Addr> = HashSet::from([
            Addr::unchecked("terra1gs8pm90pllcz5t7hsqs2fnxjv6pxzf47gg3eua"),
            Addr::unchecked("terra1rhk92dvz3tjayymy8pl08gpkmamnud7ttzc03h"),
            Addr::unchecked("terra1jwl5rzg3xe0a3f8r26l0l3d957vlxm833srtcp"),
            Addr::unchecked("terra1ng5xg5p63c3qvjpwqsmry2dz8mr9n39v8eztv9"),
            Addr::unchecked("terra1z3u6mw3s20kt67ts9txrg2mjkzcwt7tgasp7gw"),
            Addr::unchecked("terra1lnr6aacxfng34m69076s2mdfjzt8nev2p6z5q0"),
            Addr::unchecked("terra1nf6wl37jxthskggvesngsp7y50s0jchmur57v3"),
            Addr::unchecked("terra1m9hm6cxlf0907yy7lsssyfpzswlu54r99k9dxf"), /* LUART token has a transfer fee */
            Addr::unchecked("terra143az0w2e504n56q7k43qyh2fu69fh3rhup32n3"),
            Addr::unchecked("terra1ht8f0pnym9tcnt5srxmlnyndsfl84nntphqfsl"), // ???
            Addr::unchecked("terra136vrkx8kq7x5w0kgklzh9vxmn2x93q0edqe3nf"), // ???
            Addr::unchecked("terra1uyvksaz9xpgxpqnq8vz9vzg59up8mlhx3463tf"), // transfer fee
            Addr::unchecked("terra1ffjjxeu5l9f027qtkwseulgz6ddx5f04czyf0w"), // transfer fee
            Addr::unchecked("terra1t0pd5uqsqp85cgly5cvggm698asez55ked2ftu"), /* ALTE-MIR low liq, constant rounding errors? */
            Addr::unchecked("terra15asyerx0su2j3urlsn2xea0klrr5dpd75jamwj"), /* ALTE-SPEC low liq, constant rounding errors? */
            Addr::unchecked("terra1r0288hx3mjgx0zrpmpc8rulm6ca8jheqtwhwq5"), /* ALTE-STT low liq, constant rounding errors? */
            Addr::unchecked("terra1p2462m98je6pk3n03fwrnmngkm05mcr9pn6cmt"), /* ALTE-EUR low liq, constant rounding errors? */
            Addr::unchecked("terra1hkp4hulmttwzagzphr6vczvjm46e6ryxv99nan"), /* ALTE-MINE low liq, constant rounding errors? */
            Addr::unchecked("terra16eecm8exp3p82ld5eh5feweup0qvfv8468np6j"), /* ALTE-TWD low liq, constant rounding errors? */
            Addr::unchecked("terra1kh2g4fnhvqtnwwpqa84eywn72ve9vdkp5chhlx"), /* ALTE-LUNA low liq, constant rounding errors? */
            Addr::unchecked("terra1alaaant5ykususyuv0x6k298sh8kl8qkpkuu8e"), /* ALTE-aUST low liq, constant rounding errors? */
            Addr::unchecked("terra10dtff8drpwtfx4976m7cxwpptfy87l5p264xgy"), /* ALTE-ANC low liq, constant rounding errors? */
            Addr::unchecked("terra19pg6d7rrndg4z4t0jhcd7z9nhl3p5ygqttxjll"), // STT extra trade fee
            Addr::unchecked("terra1m95udvvdame93kl6j2mk8d03kc982wqgr75jsr"), // STT extra trade fee
        ]);

        Self(h)
    }
}

impl RiskClassifier for StaticPoolDenyList {
    fn exclude(&self, item: &Pool) -> bool {
        self.0.contains(&item.contract_address)
    }
}

#[derive(Default)]
pub struct UnsupportedPoolList(pub HashSet<Addr>);

impl UnsupportedPoolList {
    pub fn new() -> Self {
        let h: HashSet<Addr> = HashSet::from([
            // Stableswap pools
            Addr::unchecked("terra1wgdjvp388mlvhad8u7ly5d34ga4zyyfvf3e5j8"),
            Addr::unchecked("terra1r5m5h9nvnw0hhzwn4gzz9p2wzg78q2jsq0alvp"),
            Addr::unchecked("terra17umn0xctu4qj8w6psqfjfp20nx454csspuvdur"),
            Addr::unchecked("terra1h2g2mldq2fskq0sqdhupnzpfj396d03ds2yfkd"),
            Addr::unchecked("terra1qswfc7hmmsnwf7f2nyyx843sug60urnqgz75zu"),
            Addr::unchecked("terra1gxjjrer8mywt4020xdl5e5x7n6ncn6w38gjzae"),
            Addr::unchecked("terra1cevdyd0gvta3h79uh5t47kk235rvn42gzf0450"),
            Addr::unchecked("terra1szt6cq52akhmzcqw5jhkw3tvdjtl4kvyk3zkhx"),
            Addr::unchecked("terra1qmxkqcgcgq8ch72k6kwu3ztz6fh8tx2xd76ws7"),
            Addr::unchecked("terra1cc6kqk0yl25hdpr5llxmx62mlyfdl7n0rwl3hq"),
            Addr::unchecked("terra1x0ulpvp6m46c5j7t40nj24mjp900954ys2jsnu"),
            Addr::unchecked("terra1t59ywlkkmvy7rmkdmdpy0z7wdqez5tne9mrd0r"),
            Addr::unchecked("terra1j66jatn3k50hjtg2xemnjm8s7y8dws9xqa5y8w"),
            // ibc assets
            Addr::unchecked("terra1tq4mammgkqrxrmcfhwdz59mwvwf4qgy6rdrt46"),
            Addr::unchecked("terra194j4rhezku4dmfz2vsnpzqv0actlg3xepg7l3t"),
        ]);

        Self(h)
    }
}

impl RiskClassifier for UnsupportedPoolList {
    fn exclude(&self, item: &Pool) -> bool {
        self.0.contains(&item.contract_address)
    }
}

#[derive(Default)]
pub struct StaticPoolAllowList(pub HashSet<Addr>);

impl StaticPoolAllowList {
    pub fn new() -> Self {
        let h: HashSet<Addr> = HashSet::from([
            Addr::unchecked("terra17rvtq0mjagh37kcmm4lmpz95ukxwhcrrltgnvc"),
            Addr::unchecked("terra1l5lxw4uucaserz6f9g7av9ejyfyac5jpw94fh3"),
            Addr::unchecked("terra1afdz4l9vsqddwmjqxmel99atu4rwscpfjm4yfp"),
            Addr::unchecked("terra108ukjf6ekezuc52t9keernlqxtmzpj4wf7rx0h"),
            Addr::unchecked("terra1whns5nyc8sw328uw3qqnyafxd5yfqsytkdkgqz"),
            Addr::unchecked("terra1pn20mcwnmeyxf68vpt3cyel3n57qm9mp289jta"),
            Addr::unchecked("terra1yphapr9zkd7eq60n2nux0xfldzs58z9ncs7s6f"),
            Addr::unchecked("terra1yjg0tuhc6kzwz9jl8yqgxnf2ctwlfumnvscupp"),
            Addr::unchecked("terra1vwk0dhr5lha56ssddl58ntxlcunfru36d0jfuf"),
            Addr::unchecked("terra18dq84qfpz267xuu0k47066svuaez9hr4xvwlex"),
            Addr::unchecked("terra1uvchkwq4kv0vhy23c78hyy72zks2hqtpctklh2"),
            Addr::unchecked("terra17e0aslpj3rrt62gwh7utj3fayas4h8dl3y8ju3"),
            Addr::unchecked("terra16e5tgdxre44gvmjuu3ulsa64kc6eku4972yjp3"),
            Addr::unchecked("terra1gkdudgg2a5wt70cneyx5rtehjls4dvhhcmlptv"),
            Addr::unchecked("terra1gv6t0xkxdr096gkad4kksxr0kjl4rxuf583rgr"),
            Addr::unchecked("terra1yppvuda72pvmxd727knemvzsuergtslj486rdq"),
            Addr::unchecked("terra1zz39wfyyqt4tjz7dz6p7s9c8pwmcw2xzde3xl8"),
            Addr::unchecked("terra1l774jntnhr80wsj758fr9hlj49t8j5qf0dxhsg"),
            Addr::unchecked("terra18raj59xx32kuz66sfg82kqta6q0aslfs3m8s4r"),
            Addr::unchecked("terra1hlq6ye6km5sq2pcnmrvlf784gs9zygt0akwvsu"),
            Addr::unchecked("terra1dqjt2jm908qaayw5hdl36w50sshmgem37suawr"),
            Addr::unchecked("terra163pkeeuwxzr0yhndf8xd2jprm9hrtk59xf7nqf"),
            Addr::unchecked("terra1lr6rglgd50xxzqe6l5axaqp9d5ae3xf69z3qna"),
            Addr::unchecked("terra1k30c303e059kqhz705a77kgdhr2ndldfdxjvsr"),
            Addr::unchecked("terra1x9jv7vjcpy46k7eadp9ldx0jkkx3f8n2yvwtxe"),
            Addr::unchecked("terra1fh2j8kr484725ra3hwc9q3j52h2qdnnwargzhp"),
            Addr::unchecked("terra1etdkg9p0fkl8zal6ecp98kypd32q8k3ryced9d"),
            Addr::unchecked("terra12aazc56hv7aj2fcvmhuxve0l4pmayhpn794m0p"),
            Addr::unchecked("terra1tndcaqxkpc5ce9qee5ggqf430mr2z3pefe5wj6"),
            Addr::unchecked("terra1dkc8075nv34k2fu6xn6wcgrqlewup2qtkr4ymu"),
            Addr::unchecked("terra13ay3hftcft25uazl76q8gmdk993y9nyv9avu2h"),
            Addr::unchecked("terra1vkvmvnmex90wanque26mjvay2mdtf0rz57fm6d"),
            Addr::unchecked("terra18cxcwv0theanknfztzww8ft9pzfgkmf2xrqy23"),
            Addr::unchecked("terra1tehmd65kyleuwuf3a362mhnupkpza29vd86sml"),
            Addr::unchecked("terra1pufczag48fwqhsmekfullmyu02f93flvfc9a25"),
            Addr::unchecked("terra16j5f4lp4z8dddm3rhyw8stwrktyhcsc8ll6xtt"),
            Addr::unchecked("terra123neekasfmvcs4wa70cgw3j3uvwzqacdz2we03"),
            Addr::unchecked("terra1r6fchdsr8k65082u3cyrdn6x2n8hrpyrp72je0"),
            Addr::unchecked("terra1u56eamzkwzpm696hae4kl92jm6xxztar9uhkea"),
            Addr::unchecked("terra102a44aeeq4x78pmv5kj79qxaq2ghtsk24gj6g7"),
            Addr::unchecked("terra1n962fk7f8rfmwezadhrvxy4cyknyfd496dygrw"),
            Addr::unchecked("terra1yngadscckdtd68nzw5r5va36jccjmmasm7klpp"),
            Addr::unchecked("terra1jxzxyjf2cvl7d88un4pp9dak8tljvxhfpgrqh7"),
            Addr::unchecked("terra1gm5p3ner9x9xpwugn9sp6gvhd0lwrtkyrecdn3"),
            Addr::unchecked("terra1me6a35xuzf9ycjaaqyj7798g5tewunp6dzg27e"),
            Addr::unchecked("terra14tqq6d7n79sfjmeeqgkmewyhq65dvczgym5rhq"),
            Addr::unchecked("terra1j8cg2ncvtkfm00lvhsaa80rckq7cerhlvs9r5s"),
            Addr::unchecked("terra1zey9knmvs2frfrjnf4cfv4prc4ts3mrsefstrj"),
            Addr::unchecked("terra1u6dhkawjqxn0za9w8kvr7588ng9y73l0vj62fa"),
            Addr::unchecked("terra1d6h7dqr20lrtv4zhqntvxpxgwqw694cfvhh4st"),
            Addr::unchecked("terra1hqnk9expq3k4la2ruzdnyapgndntec4fztdyln"),
            Addr::unchecked("terra1ea9js3y4l7vy0h46k4e5r5ykkk08zc3fx7v4t8"),
            Addr::unchecked("terra1f9aln0c98j9w749szx958la7x25chdnz9hqx0v"),
            Addr::unchecked("terra1q2cg4sauyedt8syvarc8hcajw6u94ah40yp342"),
            Addr::unchecked("terra1ndvg44ygupqwq8xqzvtyqjq7agl50wwfpm0jfq"),
            Addr::unchecked("terra1jxazgm67et0ce260kvrpfv50acuushpjsz2y0p"),
            Addr::unchecked("terra1kw95x0l3qw5y7jw3j0h3fy83y56rj68wd8w7rc"),
            Addr::unchecked("terra1llhpkqd5enjfflt27u3jx0jcp5pdn6s9lfadx3"),
            Addr::unchecked("terra1cmehvqwvglg08clmqn66zfuv5cuxgxwrt3jz2u"),
            Addr::unchecked("terra1pdxyk2gkykaraynmrgjfq2uu7r9pf5v8x7k4xk"),
            Addr::unchecked("terra1ag6fqvxz33nqg78830k5c27n32mmqlcrcgqejl"),
            Addr::unchecked("terra10nfk6fcz5nc5uru964qmpels9ctg6j0vczjgl7"),
            Addr::unchecked("terra1e4vke4x2tu4xuh6s9zz42rnvghgcuxqcm3t9qx"),
            Addr::unchecked("terra19073h5d7rczrnu49r8aje5yly5llenv878eekm"),
            Addr::unchecked("terra1fxn30f9j9a2slvlxxxk6gyjlqnznhprh0wu0mv"),
            Addr::unchecked("terra1k8lvj3w7dxzd6zlyptcj086gfwms422xkqjmzx"),
            Addr::unchecked("terra1wh2jqjkagzyd3yl4sddlapy45ry808xe80fchh"),
            Addr::unchecked("terra187pxc9su35stqfdd02xy2r778qd83lruaey3fq"),
            Addr::unchecked("terra170athjxhauwzdn3tejfcv49pej3atyv4qwffrm"),
            Addr::unchecked("terra1ta3lth8wu992m20qjeucg59rlxeez6qeygsxtm"),
            Addr::unchecked("terra1aa68js6yxavg9zzzle2zaynem9cstvmaj3xyu3"),
            Addr::unchecked("terra1ng6rcmjcdcpde8jy3k37wm8kzgr7zgpqx72lud"),
            Addr::unchecked("terra1vq8ygvc47qn277pke8k3jvuyslh3ljtqkxka3m"),
            Addr::unchecked("terra1jzqlw8mfau9ewr7lufqkrpgfzk4legz9zx306p"),
            Addr::unchecked("terra15s2wgdeqhuc4gfg7sfjyaep5cch38mwtzmwqrx"),
            Addr::unchecked("terra1ltjtvt27ra3vrhyfmv4x4wymez7kwal5tzx3mq"),
            Addr::unchecked("terra1ylw3fvy4h5ll9tyaeuxrqngwnlqp3jtaegtjfq"),
            Addr::unchecked("terra1amv303y8kzxuegvurh0gug2xe9wkgj65enq2ux"),
            Addr::unchecked("terra14fyt2g3umeatsr4j4g2rs8ca0jceu3k0mcs7ry"),
            Addr::unchecked("terra1urt608par6rkcancsjzm76472phptfwq397gpm"),
            Addr::unchecked("terra1ycp5lnn0qu4sq4wq7k63zax9f05852xt9nu3yc"),
            Addr::unchecked("terra1ktsswe96kkjwvcx8md8dt4k8szt2wkr5fh7px7"),
            Addr::unchecked("terra1de8xa55xm83s3ke0s20fc5pxy7p3cpndmmm7zk"),
            Addr::unchecked("terra12mzh5cp6tgc65t2cqku5zvkjj8xjtuv5v9whyd"),
            Addr::unchecked("terra1qpd9n7afwf45rkjlpujrrdfh83pldec8rpujgn"),
            Addr::unchecked("terra1s5w4yymch3y85n54pma0qkuyqek0s5a9kys39k"),
            Addr::unchecked("terra1xskmucgxkzf3quwry3dazerw74q4aqplu0vgg4"),
            Addr::unchecked("terra1efvhm927dehrka0cgpcptt5gvjfdgqm07smawu"),
            Addr::unchecked("terra1fgc8tmys2kxzyl39k3gtgw9dlxxuhlqux7k38e"),
            Addr::unchecked("terra1h7t2yq00rxs8a78nyrnhlvp0ewu8vnfnx5efsl"),
            Addr::unchecked("terra1nfe80fnw2d0wax64ag7j95evqglqelhkh78l6t"),
            Addr::unchecked("terra132qjgv0evru0em6v2rcwakgxzafjhwfz7fc7hh"),
            Addr::unchecked("terra1v93ll6kqp33unukuwls3pslquehnazudu653au"),
            Addr::unchecked("terra134jare8j7annsu6fgtkpu6zp8gmfau88jrsnaz"),
            Addr::unchecked("terra1qr2k6yjjd5p2kaewqvg93ag74k6gyjr7re37fs"),
            Addr::unchecked("terra143xxfw5xf62d5m32k3t4eu9s82ccw80lcprzl9"),
            Addr::unchecked("terra1yfpksrlypf5c3xc9ktsup9mtxw24r6sshck3g0"),
            Addr::unchecked("terra1stdzf28wlq7llzfecse366r657rtdh6wtrdfk2"),
            Addr::unchecked("terra13f87x4c87ct5545t3j4mqw4k6jmgds5609z92c"),
            Addr::unchecked("terra1ejyqwcemr5kda5pxwz27t2ja784j3d0nj0v6lh"),
            Addr::unchecked("terra13kvcwhxa0ym2tpgcchk0rn0zrrl7r7w0mgv74q"),
            Addr::unchecked("terra16w76qmlwdevxvt9xnfafmczrjyar6rh5rtsyhw"),
            Addr::unchecked("terra14zhkur7l7ut7tx6kvj28fp5q982lrqns59mnp3"),
            Addr::unchecked("terra15xxfkxldxse9atz8ce7ne2a047pp09dy428njm"),
            Addr::unchecked("terra1ykfz4hrq8en2f098j8a4ppaq8u8hangpw9tnqu"),
            Addr::unchecked("terra180jp452au9sfwq4kuxtsd9q2wzjfu6v9ghrkax"),
            Addr::unchecked("terra1f6d3pggq7h2y7jrgwxp3xh08yhvj8znalql87h"),
            Addr::unchecked("terra1zvn8z6y8u2ndwvsjhtpsjsghk6pa6ugwzxp6vx"),
            Addr::unchecked("terra1uz8hhjwg3472744kgkz69jfqf3d2uwl5aw66mw"),
            Addr::unchecked("terra1enlwzps4sr4g84jwr3qnx3czcvf5ayjgfyfxuw"),
            Addr::unchecked("terra17mu25k7a2lj7pn8vjgtc5vjvah6gkt6njgh4q5"),
            Addr::unchecked("terra1repcset8dt8z9wm5s6x77n3sjg8hduem9tntd6"),
            Addr::unchecked("terra1ntdceldg23ymqljh94wcp78dl6qmzndq8u2hwa"),
            Addr::unchecked("terra1x8h5gan6vey5cz2xfyst74mtqsj7746fqj2hze"),
            Addr::unchecked("terra1prfcyujt9nsn5kfj5n925sfd737r2n8tk5lmpv"),
            Addr::unchecked("terra1z6tp0ruxvynsx5r9mmcc2wcezz9ey9pmrw5r8g"),
            Addr::unchecked("terra18hjdxnnkv8ewqlaqj3zpn0vsfpzdt3d0y2ufdz"),
            Addr::unchecked("terra178yhudw6cwtnrn4fq593z87y385m0xe9n6x423"),
            Addr::unchecked("terra1m6ywlgn6wrjuagcmmezzz2a029gtldhey5k552"),
            Addr::unchecked("terra14n525nzgnsqpy5ne28ffj6e2ul3c3n7hkv5ql9"),
            Addr::unchecked("terra1dgqs3aa66l24natsgslvwwkj3henn4c9uesfz9"),
            Addr::unchecked("terra1sdu63qdzkndpavvl5ephzwl7jj2slvcy89qu9s"),
            Addr::unchecked("terra15568nqrqcawm263yqcuuuvj5mh763tp8jyscq3"),
            Addr::unchecked("terra1a5cc08jt5knh0yx64pg6dtym4c4l8t63rhlag3"),
            Addr::unchecked("terra1mhtsdde76nulexf5p5fawnlnq2trs65zrd98zp"),
            Addr::unchecked("terra1hn8d8ldzu2v2td5uj335pz32phanm90a4kjfal"),
            Addr::unchecked("terra1l60336rkawujnwk7lgfq5u0s684r99p3y8hx65"),
            Addr::unchecked("terra19crn3jxwjspu29kc8ftn27lw5vr3vzk42qar86"),
            Addr::unchecked("terra18gdjzd5p76dwywsqn7kgxsadfllw6csghl37vv"),
            Addr::unchecked("terra1tq4mammgkqrxrmcfhwdz59mwvwf4qgy6rdrt46"),
            Addr::unchecked("terra134mvek3w6tp9rzgdklv9pndpdkc2gqvf24um3l"),
            Addr::unchecked("terra19wauh79y42u5vt62c5adt2g5h4exgh26t3rpds"),
            Addr::unchecked("terra14q2h9nce4spj8n74g6kppj3yf86qx8hsrqngfh"),
            Addr::unchecked("terra13yftwgefkggq3u627gphq98s6ufwh9u85h5kmg"),
            Addr::unchecked("terra1r9e7efml7zk8fd5x5atk39e8jua9kdt7n7lp5r"),
            Addr::unchecked("terra10k7y9qw63tfwj7e3x4uuzru2u9kvtd4ureajhd"),
            Addr::unchecked("terra1jkr0ef9fpghdru38ht70ds6jfldprgttw6xlek"),
            Addr::unchecked("terra1au85utmuaskgd0uhgmhus35hpaca2munlzuhrd"),
            Addr::unchecked("terra1jfp5ew4tsru98wthajsqegtzaxt49ty4z2qws0"),
            Addr::unchecked("terra1hngzkju6egu78eyzzw2fn8el9dnjk3rr704z2f"),
            Addr::unchecked("terra1d76u99glatdeep6qjkhg48u2a9fvuejcwnasut"),
            Addr::unchecked("terra1lvkkwhzgchq9n9xafag9u4q96q057vge0q87zd"),
            Addr::unchecked("terra1tus5ec9qsdht8dapq9ldfnsf8eehnfmwvsut83"),
            Addr::unchecked("terra1u9jgz9t8huvxcfc3u4w78u3fdxftcmzv297nc2"),
            Addr::unchecked("terra1sgu6yca6yjk0a34l86u6ju4apjcd6refwuhgzv"),
            Addr::unchecked("terra1x4msd26x8ncqcng3dapx8306rmgwuwtlg30a2j"),
            Addr::unchecked("terra1gq7lq389w4dxqtkxj03wp0fvz0cemj0ek5wwmm"),
            Addr::unchecked("terra16jaryra6dgfvkd3gqr5tcpy3p2s37stpa9sk7s"),
            Addr::unchecked("terra1t5prq6e8pygcyv7zwd4zs2nkxn7808fnqf5lz8"),
            Addr::unchecked("terra14wt2rhghhtuyl07h4ls78hjuh5gk4nxj66u2tp"),
            Addr::unchecked("terra18r6rdnkgrg74zew3d8l9nhk0m4xanpeukw3e20"),
            Addr::unchecked("terra1u4je05rr7galjya4pn2c6s4gutg4evmt9qlxl5"),
            Addr::unchecked("terra1runncker37zx0wlp43xchzep33udsn6yje3rt3"),
            Addr::unchecked("terra1erdxk0afwhq0hwpftndeayeqct9eluxyzt0v2e"),
            Addr::unchecked("terra1yl2atgxw422qxahm02p364wtgu7gmeya237pcs"),
            Addr::unchecked("terra1u79vnslyaxprhmeqla6lj7dkcecg800ssclhu6"),
            Addr::unchecked("terra1v5ct2tuhfqd0tf8z0wwengh4fg77kaczgf6gtx"),
            Addr::unchecked("terra15sut89ms4lts4dd5yrcuwcpctlep3hdgeu729f"),
            Addr::unchecked("terra17y0ujwl2far0rtm5qqjlx6ghtndcpyzqulur3e"),
            Addr::unchecked("terra1nzepxu4scprmc6ly4amam0f2teufy69uu02wzw"),
            Addr::unchecked("terra15nqcvvt4mag63ezmejxrdqehsydn7d3520hx7u"),
            Addr::unchecked("terra1zrzy688j8g6446jzd88vzjzqtywh6xavww92hy"),
            Addr::unchecked("terra1m32zs8725j9jzvva7zmytzasj392wpss63j2v0"),
            Addr::unchecked("terra1296jw27cq8svlg4ywm8t84u448p3zs7mcqg9ra"),
            Addr::unchecked("terra1f6d9mhrsl5t6yxqnr4rgfusjlt3gfwxdveeyuy"),
            Addr::unchecked("terra1a26j00ywq0llvms707hqycwlkl9erwhacr6jve"),
            Addr::unchecked("terra13eggta6zqfg03mxgqg9p5paqka7tgaaxnkhuuu"),
            Addr::unchecked("terra106a00unep7pvwvcck4wylt4fffjhgkf9a0u6eu"),
            Addr::unchecked("terra1z7634s8kyyvjjuv7lcgkfy49hamxssxq9f9xw6"),
            Addr::unchecked("terra15kkctr4eug9txq7v6ks6026yd4zjkrm3mc0nkp"),
            Addr::unchecked("terra1l7xu2rl3c7qmtx3r5sd2tz25glf6jh8ul7aag7"),
            Addr::unchecked("terra1edurrzv6hhd8u48engmydwhvz8qzmhhuakhwj3"),
            Addr::unchecked("terra14hklnm2ssaexjwkcfhyyyzvpmhpwx6x6lpy39s"),
            Addr::unchecked("terra1c0afrdc5253tkp5wt7rxhuj42xwyf2lcre0s7c"),
            Addr::unchecked("terra1mz0p4wzz5tmethu7rca2jjrw077hv2ypj7h06z"),
            Addr::unchecked("terra1xj2w7w8mx6m2nueczgsxy2gnmujwejjeu2xf78"),
            Addr::unchecked("terra1g8kjs70d5r68j9507s3gwymzc30yaur5j2ccfr"),
            Addr::unchecked("terra1krny2jc0tpkzeqfmswm7ss8smtddxqm3mxxsjm"),
            Addr::unchecked("terra1774f8rwx76k7ruy0gqnzq25wh7lmd72eg6eqp5"),
            Addr::unchecked("terra1476fucrvu5tuga2nx28r3fctd34xhksc2gckgf"),
            Addr::unchecked("terra1u3pknaazmmudfwxsclcfg3zy74s3zd3anc5m52"),
            Addr::unchecked("terra1fd9hxy2ajycyywzgvu72mj0m75myqy75dwuhtn"),
            Addr::unchecked("terra1mxyp5z27xxgmv70xpqjk7jvfq54as9dfzug74m"),
            Addr::unchecked("terra19fjaurx28dq4wgnf9fv3qg0lwldcln3jqafzm6"),
            Addr::unchecked("terra1uenpalqlmfaf4efgtqsvzpa3gh898d9h2a232g"),
            Addr::unchecked("terra147adm7pj9nzsdulnkqv2pnslp2en2dnpvs24qq"),
            Addr::unchecked("terra1sprg4sv9dwnk78ahxdw78asslj8upyv9lerjhm"),
            Addr::unchecked("terra1tn8ejzw8kpuc87nu42f6qeyen4c7qy35tl8t20"),
            Addr::unchecked("terra1w7hny2catfwsv6dq8gfm4zgazx3hmpl3xwzxya"),
            Addr::unchecked("terra167gwjhv4mrs0fqj0q5tejyl6cz6qc2cl95z530"),
            Addr::unchecked("terra1wr07qcmfqz2vxhcfr6k8xv8eh5es7u9mv2z07x"),
            Addr::unchecked("terra1zpnhtf9h5s7ze2ewlqyer83sr4043qcq64zfc4"),
            Addr::unchecked("terra1vayuttjw6z4hk5r734z9qatgs8vp6r5a2t043p"),
            Addr::unchecked("terra1p44kn7l233p7gcj0v3mzury8k7cwf4zt6gsxs5"),
            Addr::unchecked("terra1mp8mm0m43s926dqmwv3mgfaagvtpx8e4w84ur6"),
            Addr::unchecked("terra1nlrtp53mpvlged7pp537avj8uf59j3s0qlwrga"),
            Addr::unchecked("terra1qll0dwhllspv30xnekl2csr6k3ykygtqdzuxl8"),
            Addr::unchecked("terra12arl49w7t4xpq7krtv43t3dg6g8kn2xxyaav56"),
            Addr::unchecked("terra1zw0kfxrxgrs5l087mjm79hcmj3y8z6tljuhpmc"),
            Addr::unchecked("terra1zkyrfyq7x9v5vqnnrznn3kvj35az4f6jxftrl2"),
            Addr::unchecked("terra1upuslwv5twc8l7hrwlka4wju9z97q8ju63a6jt"),
            Addr::unchecked("terra17kmgn775v2y9m35gunn3pnw8s2ggv6h95wvkxr"),
            Addr::unchecked("terra178jydtjvj4gw8earkgnqc80c3hrmqj4kw2welz"),
            Addr::unchecked("terra1c5swgtnuunpf75klq5uztynurazuwqf0mmmcyy"),
            Addr::unchecked("terra1pmmfz205es7axymwpl4jj2ruxnevufu3462f02"),
            Addr::unchecked("terra1hmcd4kwafyydd4mjv2rzhcuuwnfuqc2prkmlhj"),
            Addr::unchecked("terra1jgvh68gc0zedgrs0fne2y8r0zvc2z7u4sdemwh"),
            Addr::unchecked("terra1y7vdguewgus669kcxjlwughyxtdt3kheys05q0"),
            Addr::unchecked("terra1z50zu7j39s2dls8k9xqyxc89305up0w7f7ec3n"),
            Addr::unchecked("terra17eakdtane6d2y7y6v0s79drq7gnhzqan48kxw7"),
            Addr::unchecked("terra10lv5wz84kpwxys7jeqkfxx299drs3vnw0lj8mz"),
            Addr::unchecked("terra10ypv4vq67ns54t5ur3krkx37th7j58paev0qhd"),
            Addr::unchecked("terra1fdwsevq7skhfq93zkg0xxr6zrv29aqupw2fura"),
            Addr::unchecked("terra1kgcv6dq7ajlh98t34puyexhy08ltnnx2tycrtu"),
            Addr::unchecked("terra1cpzkckgzz90pq8fkumdjc58ee5llrxt2yka9fp"),
            Addr::unchecked("terra1dw5j23l6nwge69z0enemutfmyc93c36aqnzjj5"),
            Addr::unchecked("terra1ze5f2lm5clq2cdd9y2ve3lglfrq6ap8cqncld8"),
            Addr::unchecked("terra10s94a5gesvayqlekgn570r3nsnmr8q7lf5zkjp"),
            Addr::unchecked("terra1v42wjhpntmq8nxjuqm6tttkx4q8zfrdd7ps3z7"),
            Addr::unchecked("terra1e59utusv5rspqsu8t37h5w887d9rdykljedxw0"),
            Addr::unchecked("terra1hm4gfvl5d65v03wzgvy5lh780dnwk6xtnaeurj"),
            Addr::unchecked("terra1fmv3g96s4xc9nrnlkdn87vxyaydd22xsxzel5r"),
            Addr::unchecked("terra1hasy32pvxmgu485x5tujylemqxynsv72lsu7ve"),
            Addr::unchecked("terra154jt8ppucvvakvqa5fyfjdflsu6v83j4ckjfq3"),
            Addr::unchecked("terra134m8n2epp0n40qr08qsvvrzycn2zq4zcpmue48"),
            Addr::unchecked("terra1je2lsw0720h2w64ca0559zcav5ltsqcv3230ze"),
            Addr::unchecked("terra1v4kpj65uq63m4x0mqzntzm27ecpactt42nyp5c"),
        ]);

        Self(h)
    }
}

impl RiskClassifier for StaticPoolAllowList {
    fn exclude(&self, item: &Pool) -> bool {
        !self.0.contains(&item.contract_address)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, str::FromStr as _};

    use cosmwasm_bignumber::Decimal256;
    use cosmwasm_std::{Addr, Uint128};
    use terrars::{
        dex::{Asset, Pool, Token},
        lcd::{self, Lcd},
    };

    use super::*;

    fn mkpool<L: Lcd>(addr: Addr) -> Pool {
        let assets = [
            Asset {
                token: Token::Native {
                    denom: "uluna".to_owned(),
                },
                amount: Uint128::new(1000000u128),
                amount_decimal: Decimal256::from_str("1.0").unwrap(),
                decimals: 6,
            },
            Asset {
                token: Token::Native {
                    denom: "uusd".to_owned(),
                },
                amount: Uint128::new(1000000u128),
                amount_decimal: Decimal256::from_str("1.0").unwrap(),
                decimals: 6,
            },
        ];
        Pool {
            contract_address: addr,
            assets,
            liquidity_token: Addr::unchecked("foobar"),
            total_share: Uint128::new(1u128),
        }
    }

    #[test]
    fn static_deny_list() {
        let h = HashSet::from([
            Addr::unchecked("terra1gs8pm90pllcz5t7hsqs2fnxjv6pxzf47gg3eua"),
            Addr::unchecked("terra1rhk92dvz3tjayymy8pl08gpkmamnud7ttzc03h"),
        ]);
        let dl = StaticPoolDenyList(h);
        let pool = mkpool::<lcd::Http>(Addr::unchecked(
            "terra1gs8pm90pllcz5t7hsqs2fnxjv6pxzf47gg3eua",
        ));
        assert!(dl.exclude(&pool));

        let pool = mkpool::<lcd::Http>(Addr::unchecked(
            "terra1yphapr9zkd7eq60n2nux0xfldzs58z9ncs7s6f",
        ));
        assert!(!dl.exclude(&pool));
    }
}
