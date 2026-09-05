# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
<style>
:root{--fam-1:#2a78d6;--fam-2:#eb6834;--fam-3:#1baf7a}
[data-md-color-scheme="slate"]{--fam-1:#3987e5;--fam-2:#d95926;--fam-3:#199e70}
@media (prefers-color-scheme:dark){:root:not([data-md-color-scheme="default"]){
  --fam-1:#3987e5;--fam-2:#d95926;--fam-3:#199e70}}
.perf-chart{position:relative}
.perf-chart>input.fam-toggle{position:absolute;width:1px;height:1px;opacity:0;
  pointer-events:none;margin:0}
.perf-chart>input.fam-toggle:nth-of-type(1):not(:checked)~figure .fam-1,
.perf-chart>input.fam-toggle:nth-of-type(2):not(:checked)~figure .fam-2,
.perf-chart>input.fam-toggle:nth-of-type(3):not(:checked)~figure .fam-3,
.perf-chart>input.fam-toggle:nth-of-type(4):not(:checked)~figure .fam-avg{
  display:none}
.perf-chart .fam-legend{display:flex;flex-wrap:wrap;gap:.35rem .9rem;
  justify-content:center;margin-top:.25rem}
.perf-chart .fam-key{cursor:pointer;user-select:none;font-size:.7rem;
  display:inline-flex;align-items:center;gap:.3rem;opacity:.45;
  border-bottom:1px dotted currentColor}
.perf-chart .fam-key::before{content:"";width:.75rem;height:0;
  border-top:2px solid currentColor}
.perf-chart .fam-key.k1{color:var(--fam-1)}
.perf-chart .fam-key.k2{color:var(--fam-2)}
.perf-chart .fam-key.k3{color:var(--fam-3)}
.perf-chart>input.fam-toggle:nth-of-type(1):checked~figure .fam-key.k1,
.perf-chart>input.fam-toggle:nth-of-type(2):checked~figure .fam-key.k2,
.perf-chart>input.fam-toggle:nth-of-type(3):checked~figure .fam-key.k3,
.perf-chart>input.fam-toggle:nth-of-type(4):checked~figure .fam-key.kavg{
  opacity:1;border-bottom-style:solid}
.perf-chart .fam-key.kavg::before{border-top-style:dashed}
</style>

### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.98 ns | 1.99 µs | 3.11 µs | 3.48 µs | 3.77 µs |
| D38 | 1.39 µs | 4.68 µs | 3.23 µs | 6.86 µs | 9.61 µs |
| D57 | 1.42 µs | 6.3 µs | 8.23 µs | 10.6 µs | 15.7 µs |
| D76 | 896 ns | 7.22 µs | 10.5 µs | 11.9 µs | 18.6 µs |
| D115 | 1.43 µs | 8.35 µs | 14.6 µs | 31.1 µs | 45.2 µs |
| D153 | 901 ns | 10.6 µs | 26.6 µs | 41.9 µs | 65.8 µs |
| D230 | 1.29 µs | 12.3 µs | 44.8 µs | 68.6 µs | 121 µs |
| D307 | 1.25 µs | 22.6 µs | 65 µs | 124 µs | 160 µs |
| D462 | 1.57 µs | 45.3 µs | 125 µs | 258 µs | 317 µs |
| D616 | 1.66 µs | 54.8 µs | 207 µs | 426 µs | 664 µs |
| D924 | 1.36 µs | 124 µs | 422 µs | 947 µs | 1.67 ms |
| D1232 | 2.04 µs | 171 µs | 659 µs | 1.65 ms | 2.76 ms |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,184.2 88.2,120.2 124.4,120.0 160.5,125.6 196.7,119.9 232.9,125.6 269.1,121.2 305.3,121.5 341.5,118.7 377.6,118.0 413.8,120.4 450.0,115.4 450.0,26.0 413.8,32.2 377.6,43.7 341.5,52.8 305.3,61.3 269.1,64.8 232.9,72.3 196.7,77.0 160.5,88.0 124.4,90.1 88.2,96.2 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.2 88.2,120.2 124.4,120.0 160.5,125.6 196.7,119.9 232.9,125.6 269.1,121.2 305.3,121.5 341.5,118.7 377.6,118.0 413.8,120.4 450.0,115.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.7 88.2,105.1 124.4,101.4 160.5,99.8 196.7,97.9 232.9,95.0 269.1,93.2 305.3,85.6 341.5,77.0 377.6,74.6 413.8,64.5 450.0,60.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.2 88.2,109.7 124.4,98.1 160.5,95.1 196.7,91.0 232.9,83.6 269.1,77.1 305.3,72.5 341.5,64.3 377.6,58.1 413.8,49.3 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,100.4 124.4,95.0 160.5,93.6 196.7,81.6 232.9,77.9 269.1,71.8 305.3,64.4 341.5,55.4 377.6,49.2 413.8,39.3 450.0,32.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,96.2 124.4,90.1 160.5,88.0 196.7,77.0 232.9,72.3 269.1,64.8 305.3,61.3 341.5,52.8 377.6,43.7 413.8,32.2 450.0,26.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.21 µs | 3.59 µs | 5.69 µs | 6.38 µs | 6.81 µs |
| D38 | 3.38 µs | 5.82 µs | 6.41 µs | 7.43 µs | 9.76 µs |
| D57 | 3.39 µs | 4.72 µs | 5.01 µs | 6.2 µs | 8.14 µs |
| D76 | 1.9 µs | 5.02 µs | 6.29 µs | 6.49 µs | 9.42 µs |
| D115 | 6.17 µs | 8.55 µs | 9.1 µs | 15.2 µs | 23.6 µs |
| D153 | 3.92 µs | 10.6 µs | 16 µs | 21.4 µs | 33.8 µs |
| D230 | 7.43 µs | 10.7 µs | 28.2 µs | 41.5 µs | 68.4 µs |
| D307 | 10.6 µs | 21.4 µs | 50.7 µs | 86 µs | 112 µs |
| D462 | 12.8 µs | 39.8 µs | 87.7 µs | 170 µs | 223 µs |
| D616 | 23.8 µs | 63.1 µs | 171 µs | 315 µs | 519 µs |
| D924 | 26 µs | 153 µs | 379 µs | 794 µs | 1.43 ms |
| D1232 | 45.6 µs | 217 µs | 674 µs | 1.56 ms | 2.42 ms |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,184.7 88.2,183.6 124.4,183.5 160.5,196.1 196.7,170.5 232.9,180.3 269.1,166.5 305.3,158.6 341.5,154.6 377.6,141.2 413.8,139.2 450.0,127.0 450.0,40.8 413.8,52.2 377.6,74.2 341.5,92.6 305.3,107.5 269.1,118.2 232.9,133.6 196.7,141.4 160.5,161.3 124.4,164.5 88.2,160.5 52.0,168.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.7 88.2,183.6 124.4,183.5 160.5,196.1 196.7,170.5 232.9,180.3 269.1,166.5 305.3,158.6 341.5,154.6 377.6,141.2 413.8,139.2 450.0,127.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.2 88.2,171.7 124.4,176.3 160.5,175.0 196.7,163.4 232.9,158.7 269.1,158.4 305.3,143.5 341.5,130.0 377.6,120.0 413.8,100.8 450.0,93.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.2 88.2,169.7 124.4,175.0 160.5,170.1 196.7,162.1 232.9,149.8 269.1,137.5 305.3,124.7 341.5,112.8 377.6,98.4 413.8,81.0 450.0,68.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.8 88.2,166.4 124.4,170.4 160.5,169.4 196.7,150.9 232.9,143.5 269.1,129.1 305.3,113.3 341.5,98.4 377.6,85.1 413.8,65.0 450.0,50.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.3 88.2,160.5 124.4,164.5 160.5,161.3 196.7,141.4 232.9,133.6 269.1,118.2 305.3,107.5 341.5,92.6 377.6,74.2 413.8,52.2 450.0,40.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 1.98 µs | 3.1 µs | 3.47 µs | 3.75 µs |
| D38 | 1.31 µs | 4.65 µs | 3.11 µs | 6.81 µs | 9.54 µs |
| D57 | 1.3 µs | 6.27 µs | 8.2 µs | 10.6 µs | 15.6 µs |
| D76 | 833 ns | 7.14 µs | 10.5 µs | 11.8 µs | 18.5 µs |
| D115 | 1.33 µs | 8.28 µs | 15.9 µs | 30.7 µs | 44.9 µs |
| D153 | 834 ns | 10.6 µs | 27.1 µs | 42.2 µs | 65 µs |
| D230 | 1.19 µs | 12.2 µs | 45.7 µs | 68.9 µs | 121 µs |
| D307 | 1.17 µs | 21.6 µs | 64.8 µs | 123 µs | 161 µs |
| D462 | 1.44 µs | 45.4 µs | 126 µs | 260 µs | 316 µs |
| D616 | 1.55 µs | 54.7 µs | 208 µs | 428 µs | 661 µs |
| D924 | 1.21 µs | 123 µs | 422 µs | 944 µs | 1.66 ms |
| D1232 | 1.97 µs | 171 µs | 656 µs | 1.65 ms | 2.75 ms |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,190.9 88.2,120.9 124.4,121.0 160.5,126.6 196.7,120.8 232.9,126.5 269.1,122.2 305.3,122.3 341.5,119.7 377.6,118.9 413.8,122.0 450.0,115.9 450.0,26.0 413.8,32.3 377.6,43.7 341.5,52.9 305.3,61.2 269.1,64.8 232.9,72.5 196.7,77.1 160.5,88.1 124.4,90.2 88.2,96.3 52.0,107.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,120.9 124.4,121.0 160.5,126.6 196.7,120.8 232.9,126.5 269.1,122.2 305.3,122.3 341.5,119.7 377.6,118.9 413.8,122.0 450.0,115.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.8 88.2,105.2 124.4,101.5 160.5,99.9 196.7,98.1 232.9,95.0 269.1,93.2 305.3,86.2 341.5,76.9 377.6,74.6 413.8,64.6 450.0,60.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.2 88.2,110.2 124.4,98.2 160.5,95.1 196.7,89.9 232.9,83.4 269.1,76.9 305.3,72.5 341.5,64.3 377.6,58.1 413.8,49.3 450.0,43.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,100.5 124.4,95.0 160.5,93.6 196.7,81.8 232.9,77.9 269.1,71.8 305.3,64.6 341.5,55.3 377.6,49.1 413.8,39.3 450.0,32.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,96.3 124.4,90.2 160.5,88.1 196.7,77.1 232.9,72.5 269.1,64.8 305.3,61.2 341.5,52.9 377.6,43.7 413.8,32.3 450.0,26.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.64 µs | 1.71 µs | 2.49 µs | 2.81 µs | 2.95 µs |
| D38 | 1.74 µs | 2.53 µs | 2.71 µs | 3.33 µs | 4.28 µs |
| D57 | 4.5 µs | 5.84 µs | 7.3 µs | 8.84 µs | 12.6 µs |
| D76 | 2.5 µs | 6.72 µs | 9.29 µs | 9.8 µs | 14.1 µs |
| D115 | 8.4 µs | 12.5 µs | 14.3 µs | 22.5 µs | 35.9 µs |
| D153 | 5.67 µs | 15.8 µs | 23.2 µs | 33.2 µs | 48 µs |
| D230 | 10.4 µs | 15.6 µs | 44.8 µs | 61.5 µs | 95 µs |
| D307 | 13.8 µs | 30.2 µs | 72.7 µs | 115 µs | 144 µs |
| D462 | 16.9 µs | 59.8 µs | 119 µs | 210 µs | 265 µs |
| D616 | 31.7 µs | 92.5 µs | 226 µs | 402 µs | 617 µs |
| D924 | 34.2 µs | 227 µs | 493 µs | 941 µs | 1.59 ms |
| D1232 | 60.7 µs | 306 µs | 864 µs | 1.76 ms | 2.45 ms |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,199.3 88.2,198.0 124.4,177.3 160.5,190.1 196.7,163.8 232.9,172.3 269.1,159.2 305.3,153.0 341.5,148.6 377.6,134.9 413.8,133.3 450.0,120.8 450.0,40.5 413.8,49.9 377.6,70.5 341.5,88.8 305.3,102.1 269.1,111.1 232.9,125.9 196.7,132.2 160.5,152.6 124.4,154.9 88.2,178.4 52.0,186.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.3 88.2,198.0 124.4,177.3 160.5,190.1 196.7,163.8 232.9,172.3 269.1,159.2 305.3,153.0 341.5,148.6 377.6,134.9 413.8,133.3 450.0,120.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.4 88.2,189.9 124.4,171.7 160.5,168.6 196.7,155.1 232.9,150.0 269.1,150.4 305.3,136.0 341.5,121.2 377.6,111.7 413.8,92.2 450.0,85.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.2 88.2,188.3 124.4,166.8 160.5,161.6 196.7,152.2 232.9,141.8 269.1,127.4 305.3,116.9 341.5,106.2 377.6,92.3 413.8,75.3 450.0,63.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.6 88.2,183.9 124.4,162.7 160.5,160.4 196.7,142.4 232.9,133.9 269.1,120.6 305.3,107.0 341.5,93.9 377.6,79.8 413.8,61.3 450.0,47.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.5 88.2,178.4 124.4,154.9 160.5,152.6 196.7,132.2 232.9,125.9 269.1,111.1 305.3,102.1 341.5,88.8 377.6,70.5 413.8,49.9 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.4 µs | 1.57 µs | 2.58 µs | 2.94 µs | 3.21 µs |
| D38 | 1.9 µs | 2.76 µs | 2.43 µs | 2.84 µs | 4.08 µs |
| D57 | 1.75 µs | 3.1 µs | 3.38 µs | 4.1 µs | 5.53 µs |
| D76 | 3.51 µs | 5.53 µs | 8.25 µs | 9.77 µs | 15.5 µs |
| D115 | 5.57 µs | 6.38 µs | 12.9 µs | 27.7 µs | 41 µs |
| D153 | 3.63 µs | 8.26 µs | 16.9 µs | 38.3 µs | 59.6 µs |
| D230 | 4.93 µs | 10.2 µs | 40.6 µs | 64.1 µs | 112 µs |
| D307 | 4.91 µs | 18.1 µs | 54.4 µs | 115 µs | 150 µs |
| D462 | 2.94 µs | 36.6 µs | 106 µs | 225 µs | 270 µs |
| D616 | 6.11 µs | 49.1 µs | 192 µs | 396 µs | 626 µs |
| D924 | 4.92 µs | 114 µs | 393 µs | 896 µs | 1.57 ms |
| D1232 | 6.36 µs | 160 µs | 619 µs | 1.58 ms | 2.63 ms |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,191.0 88.2,196.0 124.4,197.9 160.5,182.7 196.7,172.7 232.9,182.0 269.1,175.3 305.3,175.4 341.5,186.6 377.6,170.7 413.8,175.4 450.0,169.8 450.0,39.0 413.8,50.2 377.6,70.2 341.5,88.4 305.3,101.2 269.1,107.5 232.9,121.2 196.7,129.4 160.5,150.5 124.4,172.9 88.2,179.5 52.0,184.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.0 88.2,196.0 124.4,197.9 160.5,182.7 196.7,172.7 232.9,182.0 269.1,175.3 305.3,175.4 341.5,186.6 377.6,170.7 413.8,175.4 450.0,169.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,200.3 88.2,188.0 124.4,185.5 160.5,172.9 196.7,169.7 232.9,164.1 269.1,159.5 305.3,147.1 341.5,131.8 377.6,125.4 413.8,107.2 450.0,99.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.4 88.2,190.8 124.4,183.5 160.5,164.2 196.7,154.5 232.9,148.6 269.1,129.6 305.3,123.2 341.5,108.7 377.6,95.8 413.8,80.3 450.0,70.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.6 88.2,187.3 124.4,179.4 160.5,160.5 196.7,137.8 232.9,130.9 269.1,119.7 305.3,107.0 341.5,92.4 377.6,80.1 413.8,62.4 450.0,50.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.7 88.2,179.5 124.4,172.9 160.5,150.5 196.7,129.4 232.9,121.2 269.1,107.5 305.3,101.2 341.5,88.4 377.6,70.2 413.8,50.2 450.0,39.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.86 ns | 1.71 µs | 2.79 µs | 3.16 µs | 3.42 µs |
| D38 | 4.57 ns | 2.85 µs | 3.16 µs | 3.72 µs | 4.89 µs |
| D57 | 444 ns | 6.07 µs | 7.02 µs | 8.92 µs | 12 µs |
| D76 | 240 ns | 6.54 µs | 8.84 µs | 9.39 µs | 14.3 µs |
| D115 | 917 ns | 12 µs | 13.2 µs | 22.9 µs | 36.9 µs |
| D153 | 660 ns | 15 µs | 24.4 µs | 33.3 µs | 55.2 µs |
| D230 | 1.26 µs | 14.9 µs | 44 µs | 69.1 µs | 116 µs |
| D307 | 1.71 µs | 32.5 µs | 83.5 µs | 146 µs | 199 µs |
| D462 | 2.05 µs | 61.7 µs | 149 µs | 303 µs | 406 µs |
| D616 | 3.75 µs | 97.4 µs | 299 µs | 566 µs | 951 µs |
| D924 | 3.76 µs | 254 µs | 675 µs | 1.47 ms | 2.68 ms |
| D1232 | 7.2 µs | 383 µs | 1.22 ms | 2.89 ms | 4.52 ms |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,193.2 88.2,191.1 124.4,134.4 160.5,142.0 196.7,125.4 232.9,129.4 269.1,121.4 305.3,117.6 341.5,115.4 377.6,107.9 413.8,107.9 450.0,99.8 450.0,19.9 413.8,26.3 377.6,39.2 341.5,49.8 305.3,58.6 269.1,65.3 232.9,74.5 196.7,79.5 160.5,91.3 124.4,93.4 88.2,104.6 52.0,109.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.2 88.2,191.1 124.4,134.4 160.5,142.0 196.7,125.4 232.9,129.4 269.1,121.4 305.3,117.6 341.5,115.4 377.6,107.9 413.8,107.9 450.0,99.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.6 88.2,111.3 124.4,101.9 160.5,101.0 196.7,93.5 232.9,90.7 269.1,90.8 305.3,81.1 341.5,73.1 377.6,67.5 413.8,55.6 450.0,50.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.5 88.2,110.0 124.4,100.1 160.5,97.2 196.7,92.2 232.9,84.7 269.1,77.3 305.3,69.4 341.5,62.2 377.6,53.6 413.8,43.5 450.0,36.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.0 88.2,108.0 124.4,97.1 160.5,96.5 196.7,85.4 232.9,80.8 269.1,71.7 305.3,62.4 341.5,53.4 377.6,45.6 413.8,33.8 450.0,25.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.0 88.2,104.6 124.4,93.4 160.5,91.3 196.7,79.5 232.9,74.5 269.1,65.3 305.3,58.6 341.5,49.8 377.6,39.2 413.8,26.3 450.0,19.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.26 µs | 1.18 µs | 2.11 µs | 2.62 µs | 2.84 µs |
| D38 | 1.35 µs | 2.51 µs | 2.45 µs | 2.74 µs | 3.83 µs |
| D57 | 2.9 µs | 3.73 µs | 4.71 µs | 5.64 µs | 9.62 µs |
| D76 | 1.87 µs | 4.04 µs | 5.87 µs | 6.01 µs | 9.07 µs |
| D115 | 2.98 µs | 4.68 µs | 7.14 µs | 12.7 µs | 19 µs |
| D153 | 1.87 µs | 5.84 µs | 10.1 µs | 18.6 µs | 32.2 µs |
| D230 | 2.67 µs | 6.19 µs | 20.8 µs | 35.6 µs | 69.5 µs |
| D307 | 2.68 µs | 10.7 µs | 27.6 µs | 68.8 µs | 94.7 µs |
| D462 | 1.96 µs | 15.9 µs | 62.2 µs | 143 µs | 183 µs |
| D616 | 3.37 µs | 26.4 µs | 123 µs | 259 µs | 429 µs |
| D924 | 2.67 µs | 70.2 µs | 256 µs | 615 µs | 1.12 ms |
| D1232 | 3.76 µs | 103 µs | 424 µs | 1.13 ms | 1.92 ms |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,205.0 88.2,203.5 124.4,186.9 160.5,196.4 196.7,186.3 232.9,196.4 269.1,188.6 305.3,188.6 341.5,195.4 377.6,183.6 413.8,188.7 450.0,181.3 450.0,45.8 413.8,57.5 377.6,78.4 341.5,96.9 305.3,111.2 269.1,117.9 232.9,134.6 196.7,146.0 160.5,162.1 124.4,160.8 88.2,180.8 52.0,187.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,205.0 88.2,203.5 124.4,186.9 160.5,196.4 196.7,186.3 232.9,196.4 269.1,188.6 305.3,188.6 341.5,195.4 377.6,183.6 413.8,188.7 450.0,181.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,206.4 88.2,190.0 124.4,181.4 160.5,179.7 196.7,176.5 232.9,171.7 269.1,170.4 305.3,158.6 341.5,149.9 377.6,138.9 413.8,117.7 450.0,109.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,193.7 88.2,190.5 124.4,176.4 160.5,171.6 196.7,167.3 232.9,159.7 269.1,144.1 305.3,138.0 341.5,120.3 377.6,105.5 413.8,89.6 450.0,78.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.1 88.2,188.1 124.4,172.5 160.5,171.1 196.7,154.7 232.9,146.5 269.1,132.4 305.3,118.1 341.5,102.3 377.6,89.4 413.8,70.6 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.4 88.2,180.8 124.4,160.8 160.5,162.1 196.7,146.0 232.9,134.6 269.1,117.9 305.3,111.2 341.5,96.9 377.6,78.4 413.8,57.5 450.0,45.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.56 µs | 1.72 µs | 2.93 µs | 3.54 µs | 3.76 µs |
| D38 | 1.71 µs | 3.3 µs | 3.26 µs | 3.75 µs | 4.86 µs |
| D57 | 5.68 µs | 5.94 µs | 7.27 µs | 7.99 µs | 11 µs |
| D76 | 3.53 µs | 6.31 µs | 8.1 µs | 8.56 µs | 12.2 µs |
| D115 | 12.6 µs | 12.3 µs | 8.97 µs | 20.1 µs | 28.9 µs |
| D153 | 3.68 µs | 8.32 µs | 16.8 µs | 23.2 µs | 39.1 µs |
| D230 | 5.15 µs | 8.94 µs | 24.5 µs | 42.1 µs | 79.7 µs |
| D307 | 5.31 µs | 14.3 µs | 53 µs | 80.4 µs | 102 µs |
| D462 | 6.16 µs | 25 µs | 82.5 µs | 169 µs | 197 µs |
| D616 | 6.74 µs | 31.8 µs | 134 µs | 271 µs | 418 µs |
| D924 | 5.41 µs | 82.5 µs | 271 µs | 612 µs | 994 µs |
| D1232 | 7.61 µs | 113 µs | 411 µs | 1 ms | 2.2 ms |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,200.4 88.2,198.3 124.4,172.3 160.5,182.6 196.7,154.9 232.9,181.7 269.1,174.4 305.3,173.7 341.5,170.5 377.6,168.6 413.8,173.3 450.0,165.9 450.0,42.8 413.8,60.1 377.6,78.9 341.5,95.3 305.3,109.5 269.1,114.9 232.9,130.4 196.7,137.0 160.5,155.7 124.4,157.9 88.2,175.6 52.0,181.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,200.4 88.2,198.3 124.4,172.3 160.5,182.6 196.7,154.9 232.9,181.7 269.1,174.4 305.3,173.7 341.5,170.5 377.6,168.6 413.8,173.3 450.0,165.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.3 88.2,184.0 124.4,171.3 160.5,170.0 196.7,155.5 232.9,164.0 269.1,162.4 305.3,152.2 341.5,140.1 377.6,134.8 413.8,114.2 450.0,107.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.7 88.2,184.3 124.4,166.9 160.5,164.6 196.7,162.4 232.9,148.7 269.1,140.6 305.3,123.8 341.5,114.2 377.6,103.6 413.8,88.3 450.0,79.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.6 88.2,181.3 124.4,164.9 160.5,163.4 196.7,144.9 232.9,141.7 269.1,128.8 305.3,114.7 341.5,98.6 377.6,88.3 413.8,70.6 450.0,60.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.3 88.2,175.6 124.4,157.9 160.5,155.7 196.7,137.0 232.9,130.4 269.1,114.9 305.3,109.5 341.5,95.3 377.6,78.9 413.8,60.1 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.34 µs | 1.07 µs | 1.96 µs | 2.58 µs | 2.8 µs |
| D38 | 1.44 µs | 2.35 µs | 2.07 µs | 2.59 µs | 3.8 µs |
| D57 | 3.11 µs | 3.52 µs | 4.43 µs | 5.55 µs | 9.63 µs |
| D76 | 1.97 µs | 3.85 µs | 5.56 µs | 6 µs | 8.75 µs |
| D115 | 3.3 µs | 4.48 µs | 7.38 µs | 12.1 µs | 19.2 µs |
| D153 | 1.99 µs | 5.6 µs | 10.2 µs | 18 µs | 31.7 µs |
| D230 | 2.82 µs | 6.1 µs | 19.5 µs | 35.3 µs | 65.5 µs |
| D307 | 2.79 µs | 10.3 µs | 26 µs | 66.4 µs | 93.2 µs |
| D462 | 1.89 µs | 16.3 µs | 60.2 µs | 142 µs | 183 µs |
| D616 | 3.55 µs | 25.7 µs | 120 µs | 255 µs | 427 µs |
| D924 | 2.83 µs | 67.3 µs | 255 µs | 610 µs | 1.12 ms |
| D1232 | 3.83 µs | 100 µs | 423 µs | 1.13 ms | 1.92 ms |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,203.6 88.2,202.1 124.4,185.4 160.5,195.3 196.7,184.1 232.9,195.0 269.1,187.5 305.3,187.7 341.5,196.2 377.6,182.5 413.8,187.4 450.0,180.9 450.0,45.9 413.8,57.6 377.6,78.5 341.5,96.9 305.3,111.5 269.1,119.2 232.9,135.0 196.7,145.9 160.5,162.9 124.4,160.8 88.2,181.0 52.0,187.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,203.6 88.2,202.1 124.4,185.4 160.5,195.3 196.7,184.1 232.9,195.0 269.1,187.5 305.3,187.7 341.5,196.2 377.6,182.5 413.8,187.4 450.0,180.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,208.6 88.2,191.5 124.4,182.7 160.5,180.7 196.7,177.4 232.9,172.6 269.1,170.7 305.3,159.3 341.5,149.4 377.6,139.5 413.8,118.6 450.0,109.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,195.3 88.2,194.2 124.4,177.7 160.5,172.8 196.7,166.6 232.9,159.5 269.1,145.5 305.3,139.3 341.5,121.0 377.6,106.0 413.8,89.7 450.0,78.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.4 88.2,189.3 124.4,172.8 160.5,171.1 196.7,155.9 232.9,147.2 269.1,132.6 305.3,118.9 341.5,102.5 377.6,89.7 413.8,70.7 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.7 88.2,181.0 124.4,160.8 160.5,162.9 196.7,145.9 232.9,135.0 269.1,119.2 305.3,111.5 341.5,96.9 377.6,78.5 413.8,57.6 450.0,45.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.57 µs | 1.72 µs | 2.93 µs | 3.55 µs | 3.77 µs |
| D38 | 1.71 µs | 3.31 µs | 3.19 µs | 3.76 µs | 4.87 µs |
| D57 | 5.69 µs | 5.93 µs | 7.27 µs | 8.03 µs | 11 µs |
| D76 | 3.55 µs | 6.32 µs | 8.09 µs | 8.56 µs | 12.3 µs |
| D115 | 12.6 µs | 12.2 µs | 8.73 µs | 20.1 µs | 28.3 µs |
| D153 | 3.7 µs | 8.36 µs | 16.2 µs | 23.3 µs | 39.6 µs |
| D230 | 5.18 µs | 8.97 µs | 24.5 µs | 42.2 µs | 79.9 µs |
| D307 | 5.31 µs | 14.2 µs | 53.2 µs | 80.6 µs | 102 µs |
| D462 | 6.27 µs | 25.2 µs | 82.5 µs | 168 µs | 199 µs |
| D616 | 6.93 µs | 31.9 µs | 135 µs | 271 µs | 418 µs |
| D924 | 5.53 µs | 82.6 µs | 273 µs | 614 µs | 996 µs |
| D1232 | 7.65 µs | 114 µs | 411 µs | 1 ms | 2.21 ms |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,200.2 88.2,198.3 124.4,172.3 160.5,182.5 196.7,154.9 232.9,181.6 269.1,174.3 305.3,173.8 341.5,170.1 377.6,168.0 413.8,172.9 450.0,165.8 450.0,42.8 413.8,60.1 377.6,78.9 341.5,95.0 305.3,109.5 269.1,114.9 232.9,130.1 196.7,137.4 160.5,155.6 124.4,157.9 88.2,175.6 52.0,181.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,200.2 88.2,198.3 124.4,172.3 160.5,182.5 196.7,154.9 232.9,181.6 269.1,174.3 305.3,173.8 341.5,170.1 377.6,168.0 413.8,172.9 450.0,165.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.2 88.2,184.0 124.4,171.4 160.5,170.0 196.7,155.7 232.9,163.9 269.1,162.4 305.3,152.4 341.5,140.0 377.6,134.8 413.8,114.1 450.0,107.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.6 88.2,184.8 124.4,166.9 160.5,164.6 196.7,163.0 232.9,149.5 269.1,140.5 305.3,123.7 341.5,114.2 377.6,103.5 413.8,88.2 450.0,79.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.5 88.2,181.2 124.4,164.8 160.5,163.4 196.7,144.8 232.9,141.6 269.1,128.7 305.3,114.7 341.5,98.7 377.6,88.3 413.8,70.6 450.0,59.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.2 88.2,175.6 124.4,157.9 160.5,155.6 196.7,137.4 232.9,130.1 269.1,114.9 305.3,109.5 341.5,95.0 377.6,78.9 413.8,60.1 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.49 µs | 2.13 µs | 3.6 µs | 4.24 µs | 4.61 µs |
| D38 | 2.65 µs | 3.98 µs | 4.21 µs | 4.73 µs | 6.2 µs |
| D57 | 3.99 µs | 4.67 µs | 5.92 µs | 7.18 µs | 9.97 µs |
| D76 | 2.51 µs | 5.01 µs | 7.33 µs | 7.58 µs | 10.9 µs |
| D115 | 3.96 µs | 5.9 µs | 9.13 µs | 14.9 µs | 23.2 µs |
| D153 | 2.55 µs | 7.44 µs | 12.8 µs | 21.4 µs | 36.7 µs |
| D230 | 3.54 µs | 7.7 µs | 23.3 µs | 40.5 µs | 74.1 µs |
| D307 | 3.59 µs | 12.4 µs | 32 µs | 75.7 µs | 102 µs |
| D462 | 2.44 µs | 20.2 µs | 68.2 µs | 156 µs | 199 µs |
| D616 | 4.51 µs | 30.1 µs | 134 µs | 279 µs | 463 µs |
| D924 | 3.63 µs | 76.3 µs | 278 µs | 658 µs | 1.2 ms |
| D1232 | 4.8 µs | 112 µs | 461 µs | 1.2 ms | 2.02 ms |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,190.2 88.2,188.8 124.4,179.9 160.5,190.0 196.7,180.1 232.9,189.7 269.1,182.5 305.3,182.2 341.5,190.6 377.6,177.3 413.8,182.0 450.0,175.9 450.0,44.7 413.8,56.0 377.6,76.7 341.5,95.0 305.3,109.5 269.1,116.5 232.9,131.8 196.7,141.7 160.5,158.1 124.4,160.1 88.2,170.4 52.0,176.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.2 88.2,188.8 124.4,179.9 160.5,190.0 196.7,180.1 232.9,189.7 269.1,182.5 305.3,182.2 341.5,190.6 377.6,177.3 413.8,182.0 450.0,175.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,193.6 88.2,180.0 124.4,176.5 160.5,175.0 196.7,171.5 232.9,166.4 269.1,165.7 305.3,155.4 341.5,144.7 377.6,136.1 413.8,115.9 450.0,107.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.2 88.2,178.8 124.4,171.4 160.5,166.8 196.7,162.0 232.9,154.6 269.1,141.7 305.3,134.8 341.5,118.3 377.6,103.7 413.8,87.8 450.0,76.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.6 88.2,176.3 124.4,167.2 160.5,166.0 196.7,151.3 232.9,143.5 269.1,129.6 305.3,116.1 341.5,100.4 377.6,87.7 413.8,69.1 450.0,56.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,176.8 88.2,170.4 124.4,160.1 160.5,158.1 196.7,141.7 232.9,131.8 269.1,116.5 305.3,109.5 341.5,95.0 377.6,76.7 413.8,56.0 450.0,44.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.66 µs | 1.74 µs | 3.09 µs | 3.71 µs | 3.94 µs |
| D38 | 1.82 µs | 3.47 µs | 3.28 µs | 3.75 µs | 4.91 µs |
| D57 | 5.81 µs | 6.13 µs | 7.62 µs | 8.49 µs | 11.5 µs |
| D76 | 3.64 µs | 6.5 µs | 8.44 µs | 8.87 µs | 12.8 µs |
| D115 | 13.1 µs | 12.7 µs | 9.29 µs | 20.9 µs | 29.4 µs |
| D153 | 3.81 µs | 8.73 µs | 17.1 µs | 23.8 µs | 39.8 µs |
| D230 | 5.27 µs | 9.48 µs | 25.3 µs | 43.2 µs | 81.3 µs |
| D307 | 5.4 µs | 15.1 µs | 55.8 µs | 81.8 µs | 108 µs |
| D462 | 6.45 µs | 25.9 µs | 84.2 µs | 173 µs | 200 µs |
| D616 | 7.12 µs | 33.3 µs | 137 µs | 275 µs | 422 µs |
| D924 | 5.82 µs | 83.6 µs | 275 µs | 615 µs | 1 ms |
| D1232 | 8 µs | 115 µs | 416 µs | 1.01 ms | 2.22 ms |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,199.0 88.2,197.0 124.4,171.8 160.5,182.0 196.7,154.2 232.9,181.0 269.1,173.9 305.3,173.4 341.5,169.5 377.6,167.4 413.8,171.8 450.0,164.8 450.0,42.7 413.8,60.0 377.6,78.7 341.5,94.9 305.3,108.4 269.1,114.5 232.9,130.0 196.7,136.6 160.5,154.6 124.4,157.0 88.2,175.5 52.0,180.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.0 88.2,197.0 124.4,171.8 160.5,182.0 196.7,154.2 232.9,181.0 269.1,173.9 305.3,173.4 341.5,169.5 377.6,167.4 413.8,171.8 450.0,164.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.9 88.2,183.0 124.4,170.6 160.5,169.4 196.7,154.7 232.9,163.0 269.1,161.2 305.3,151.1 341.5,139.3 377.6,133.9 413.8,113.9 450.0,107.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,185.5 88.2,184.2 124.4,165.9 160.5,163.7 196.7,161.6 232.9,148.4 269.1,139.9 305.3,122.7 341.5,113.7 377.6,103.2 413.8,88.0 450.0,79.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.5 88.2,181.3 124.4,163.6 160.5,162.6 196.7,143.9 232.9,141.2 269.1,128.2 305.3,114.4 341.5,98.2 377.6,88.0 413.8,70.5 450.0,59.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.2 88.2,175.5 124.4,157.0 160.5,154.6 196.7,136.6 232.9,130.0 269.1,114.5 305.3,108.4 341.5,94.9 377.6,78.7 413.8,60.0 450.0,42.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 136 ns | 135 ns | 157 ns | 167 ns | 170 ns |
| D38 | 147 ns | 160 ns | 125 ns | 165 ns | 188 ns |
| D57 | 233 ns | 336 ns | 320 ns | 323 ns | 448 ns |
| D76 | 126 ns | 328 ns | 354 ns | 344 ns | 445 ns |
| D115 | 509 ns | 570 ns | 479 ns | 677 ns | 984 ns |
| D153 | 262 ns | 695 ns | 798 ns | 859 ns | 1.13 µs |
| D230 | 568 ns | 566 ns | 1.09 µs | 1.26 µs | 1.82 µs |
| D307 | 812 ns | 994 ns | 1.7 µs | 2.11 µs | 2.27 µs |
| D462 | 1.05 µs | 1.62 µs | 2.2 µs | 3.47 µs | 3.37 µs |
| D616 | 1.38 µs | 1.7 µs | 2.97 µs | 4.21 µs | 5.73 µs |
| D924 | 1.29 µs | 2.81 µs | 4.57 µs | 7.68 µs | 11.3 µs |
| D1232 | 2.66 µs | 3.56 µs | 7.14 µs | 12.1 µs | 23.6 µs |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,201.0 88.2,198.9 124.4,185.5 160.5,203.4 196.7,162.9 232.9,182.1 269.1,159.7 305.3,149.4 341.5,142.1 377.6,133.9 413.8,136.0 450.0,115.0 450.0,51.8 413.8,73.0 377.6,92.8 341.5,108.2 305.3,119.6 269.1,126.0 232.9,139.7 196.7,143.8 160.5,166.8 124.4,166.6 88.2,191.8 52.0,194.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,201.0 88.2,198.9 124.4,185.5 160.5,203.4 196.7,162.9 232.9,182.1 269.1,159.7 305.3,149.4 341.5,142.1 377.6,133.9 413.8,136.0 450.0,115.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,201.3 88.2,196.4 124.4,174.9 160.5,175.6 196.7,159.6 232.9,153.9 269.1,159.8 305.3,143.5 341.5,129.3 377.6,127.9 413.8,113.4 450.0,106.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,196.9 88.2,203.6 124.4,176.4 160.5,173.4 196.7,164.7 232.9,149.9 269.1,140.8 305.3,128.0 341.5,120.5 377.6,111.9 413.8,99.3 450.0,86.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,195.1 88.2,195.5 124.4,176.0 160.5,174.2 196.7,154.6 232.9,147.7 269.1,136.6 305.3,121.7 341.5,107.3 377.6,101.7 413.8,84.3 450.0,71.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,194.7 88.2,191.8 124.4,166.6 160.5,166.8 196.7,143.8 232.9,139.7 269.1,126.0 305.3,119.6 341.5,108.2 377.6,92.8 413.8,73.0 450.0,51.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 154 ns | 154 ns | 192 ns | 201 ns | 201 ns |
| D38 | 166 ns | 189 ns | 188 ns | 186 ns | 200 ns |
| D57 | 325 ns | 402 ns | 383 ns | 396 ns | 540 ns |
| D76 | 166 ns | 404 ns | 444 ns | 403 ns | 538 ns |
| D115 | 655 ns | 688 ns | 535 ns | 801 ns | 1.06 µs |
| D153 | 340 ns | 796 ns | 928 ns | 952 ns | 1.29 µs |
| D230 | 745 ns | 646 ns | 1.23 µs | 1.46 µs | 2.01 µs |
| D307 | 1.1 µs | 1.21 µs | 1.97 µs | 2.44 µs | 2.62 µs |
| D462 | 1.39 µs | 1.87 µs | 2.56 µs | 3.7 µs | 3.72 µs |
| D616 | 1.79 µs | 1.84 µs | 3.35 µs | 4.66 µs | 6.2 µs |
| D924 | 1.66 µs | 3.26 µs | 5.07 µs | 8.22 µs | 12 µs |
| D1232 | 3.37 µs | 4.07 µs | 7.8 µs | 13 µs | 24.5 µs |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,197.5 88.2,195.4 124.4,175.8 160.5,195.3 196.7,155.6 232.9,174.6 269.1,151.8 305.3,140.6 341.5,133.8 377.6,126.5 413.8,128.6 450.0,108.2 450.0,50.7 413.8,71.5 377.6,90.5 341.5,105.3 305.3,115.4 269.1,123.1 232.9,136.0 196.7,141.6 160.5,161.3 124.4,161.2 88.2,189.9 52.0,189.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.5 88.2,195.4 124.4,175.8 160.5,195.3 196.7,155.6 232.9,174.6 269.1,151.8 305.3,140.6 341.5,133.8 377.6,126.5 413.8,128.6 450.0,108.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.4 88.2,191.6 124.4,169.7 160.5,169.6 196.7,154.2 232.9,149.9 269.1,156.0 305.3,137.9 341.5,125.3 377.6,125.7 413.8,109.1 450.0,102.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.1 88.2,191.8 124.4,171.1 160.5,166.8 196.7,161.4 232.9,145.5 269.1,137.4 305.3,123.6 341.5,116.1 377.6,108.3 413.8,96.4 450.0,83.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.8 88.2,192.0 124.4,170.2 160.5,169.7 196.7,149.7 232.9,144.7 269.1,132.3 305.3,117.5 341.5,105.5 377.6,98.8 413.8,82.3 450.0,69.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.9 88.2,189.9 124.4,161.2 160.5,161.3 196.7,141.6 232.9,136.0 269.1,123.1 305.3,115.4 341.5,105.3 377.6,90.5 413.8,71.5 450.0,50.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>
<!-- END GENERATED:performance:body:trig -->
