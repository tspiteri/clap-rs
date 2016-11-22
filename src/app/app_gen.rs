use std::io;

use app::Parser;

pub struct AppGen<'a, 'b>
    where 'a: 'b {
    p: &'b mut Parser<'a, 'b>
}

impl<'a, 'b> AppGen<'a, 'b, W> {

    pub fn new(p: &mut Parser<'a, 'b>) -> Self {
        AppGen {
            p: p
        }
    }

    pub fn generate<W: io::Write>(&self, w: &mut W) {
        self.gen_meta(w);
        write!(w, "
            pub fn {bn}() -> ::clap::App {
                let mut flags = Vec::with_capacity({fc});
                let mut opts = Vec::with_capacity({oc});
                let mut pos = VecMap::with_capacity({pc});
                let mut sc = Vec::with_capacity({scc});
                let mut reqs = Vec::with_capacity({rc});
                let mut bl = Vec::with_capacity({bc});
                let mut ol = Vec::with_capacity({olc});
                let mut sl = Vec::with_capacity({slc});
                let mut ll = Vec::with_capacity({llc});
                let mut grps = HashMap::with_capacity({gc});
                let mut ga = Vec::with_capacity({gac});
                App {
                    p: Parser {
                        flags: flags,
                        opts: opts,
                        positionals: pos,
                        subcommands: sc,
                        help_short: Some(hs),
                        version_short: Some(vs),
                        required: reqs,
                        blacklist: bl,
                        overrides: ol,
                        short_list: sl,
                        long_list: ll,
                        groups: grps,
                        global_args: ga,
                        g_settings: AppFlags::from({gset}),
                        settings: AppFlags::from({set}),
                        meta: {bn}_meta(),
                        trailing_vals: false,
                        id: 0,
                        valid_neg_num: false,
                    },
                }
            }
        ",
            bn=self.p.meta.bin_name.as_ref().expect(INTERNAL_ERROR_MSG),
            fc=self.p.flags.len(),
            oc=self.p.opts.len(),
            pc=self.p.positionals.len(),
            scc=self.p.subcmds.len(),
            rc=self.required.len(),
            bc=self.blacklist.len(),
            olc=self.overrides.len(),
            slc=self.short_list.len(),
            llc=self.long_list.len(),
            gc=self.groups.len(),
            gac=self.global_args.len()
            gset=self.g_settings.as_u32(),
            set=self.settings.as_u32(),
            ).expect(INTERNAL_ERROR_MSG)
    }

    fn gen_meta<W: io::Write>(&self, w: &mut W) {
        write!(w, "
        fn {bn}_meta() -> AppMeta {
            let mut als = Vec::with_capacity({ac});
            AppMeta {
                name: {name},
                author: {author},
                about: {about},
                more_help: {more_help},
                pre_help: {pre_help},
                version: {version},
                usage_str: {usage_str},
                usage: {usage},
                bin_name: {bin_name},
                help_str: {help_str},
                disp_ord: {disp_ord},
                template: {template},
                aliases: als,
                term_w: {term_w},
                max_w: {max_w},
            }
        }
        ",
            bn=,
            ac=,
            name=self.p.meta.name,
            author=format!("{:?}",self.p.meta.author),
            about=format!("{:?}",self.p.meta.about),
            more_help=format!("{:?}",self.p.meta.more_help),
            pre_help=format!("{:?}",self.p.meta.pre_help),
            version=format!("{:?}",self.p.meta.version),
            usage_str=format!("{:?}",self.p.meta.usage_str),
            usage=format!("{:?}",self.p.meta.usage),
            bin_name=format!("{:?}",self.p.meta.bin_name)
            help_str=format!("{:?}",self.p.meta.help_str),
            disp_ord=format!("{:?}",self.p.meta.disp_ord),
            template=format!("{:?}",self.p.meta.template),
            term_w=format!("{:?}",self.p.meta.term_w),
            max_w=format!("{:?}",self.p.meta.max_w),
            ).expect(INTERNAL_ERROR_MSG)
    }
}