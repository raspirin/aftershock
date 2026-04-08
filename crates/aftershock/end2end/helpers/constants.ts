export const STORAGE_URL = process.env.STORAGE_URL ?? "http://127.0.0.1:3030";
export const FRONTEND_URL = process.env.FRONTEND_URL ?? "http://127.0.0.1:3000";
export const SITE_TITLE = "破碎镜隙映影";

/** Error / placeholder messages matching Rust consts.rs */
export const MSG = {
  loadFailure: "无法从破碎镜隙映影中取回你想要的讯息。",
  notFound: "破碎镜隙映影中无法找到你想要的讯息。",
  archivePlaceholder: "正在从破碎镜隙映影中整理你想要的讯息。",
};

export const SEL = {
  header: "header",
  nav: "header nav",
  main: "main",
  siteTitle: `header a[title="${SITE_TITLE}"]`,
  article: "article",
  yearHeading: "section > h1",
  postTitle: "section h2",
  tagLink: 'a[href^="/tags/"]',
  prose: "div.prose",
  licenseLink: 'a[title="CC BY-NC-SA"]',
  headerLine: "header .header-line",
  messageBox: "div.rounded-lg.shadow-md",
};
