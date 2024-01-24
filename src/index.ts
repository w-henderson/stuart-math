import { mathjax } from "mathjax-full/js/mathjax.js";
import { TeX } from "mathjax-full/js/input/tex.js";
import { SVG } from "mathjax-full/js/output/svg.js";
import { liteAdaptor } from "mathjax-full/js/adaptors/liteAdaptor.js";
import { RegisterHTMLHandler } from "mathjax-full/js/handlers/html.js";
import { AllPackages } from "mathjax-full/js/input/tex/AllPackages.js";
import type { LiteElement } from "mathjax-full/js/adaptors/lite/Element";

const adaptor = liteAdaptor();
RegisterHTMLHandler(adaptor);

export default {
  name: "stuart_math",
  version: "0.2.0",
  functions: [
    {
      name: "math",
      fn: (latex: string) => math(latex, true),
    },
    {
      name: "mathi",
      fn: (latex: string) => math(latex, false)
    }
  ]
};

function math(latex: string, display: boolean): string {
  const tex = new TeX({ packages: AllPackages });
  const svg = new SVG();
  const doc = mathjax.document("", { InputJax: tex, OutputJax: svg });
  const node: LiteElement = doc.convert(latex, { display }).children[0];

  if (display) {
    adaptor.setStyle(node, "display", "block");
    adaptor.setStyle(node, "margin", "auto");
  }

  const svgString = adaptor.outerHTML(node);

  if (svgString.includes("data-mjx-error")) {
    const errorTitle = svgString.match(/title="([^"]+)"/)![1];
    throw new Error(errorTitle);
  }

  return svgString;
}