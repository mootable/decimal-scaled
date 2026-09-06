# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
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

### `add`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `add@base` D18 | 1.04 ns | 1.04 ns | 0.935 ns | 0.937 ns | 1.06 ns |
| `add@base` D38 | 1.82 ns | 1.62 ns | 1.44 ns | 1.14 ns | 0.994 ns |
| `add@base` D57 | 2.28 ns | 1.94 ns | 2.02 ns | 2.28 ns | 2.28 ns |
| `add@base` D76 | 3.49 ns | 3.09 ns | 1.88 ns | 3.09 ns | 2.08 ns |
| `add@base` D115 | 3.57 ns | 4.41 ns | 4.41 ns | 5.01 ns | 4.99 ns |
| `add@base` D153 | 3.78 ns | 5.89 ns | 6.65 ns | 5.89 ns | 5.89 ns |
| `add@base` D230 | 13.9 ns | 15.3 ns | 10.1 ns | 10.1 ns | 13.9 ns |
| `add@base` D307 | 14.6 ns | 18.5 ns | 18.5 ns | 19.5 ns | 18.5 ns |
| `add@base` D462 | 32.6 ns | 29 ns | 27.3 ns | 22.8 ns | 23 ns |
| `add@base` D616 | 50.3 ns | 51.7 ns | 40.2 ns | 50.5 ns | 33.9 ns |
| `add@base` D924 | 84.8 ns | 75.8 ns | 67.1 ns | 85.3 ns | 55.1 ns |
| `add@base` D1232 | 94.9 ns | 95.1 ns | 95 ns | 104 ns | 106 ns |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-add-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,159.1 88.2,147.0 124.4,142.1 160.5,132.8 196.7,132.3 232.9,131.1 269.1,102.9 305.3,101.8 341.5,84.3 377.6,74.9 413.8,63.6 450.0,61.1 450.0,58.7 413.8,72.9 377.6,83.5 341.5,92.0 305.3,96.6 269.1,102.9 232.9,121.5 196.7,125.1 160.5,144.1 124.4,142.1 88.2,160.1 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,159.1 88.2,147.0 124.4,142.1 160.5,132.8 196.7,132.3 232.9,131.1 269.1,102.9 305.3,101.8 341.5,84.3 377.6,74.9 413.8,63.6 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,159.1 88.2,149.5 124.4,145.6 160.5,135.5 196.7,127.8 232.9,121.5 269.1,100.7 305.3,96.6 341.5,86.9 377.6,74.3 413.8,66.0 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,152.1 124.4,144.7 160.5,146.2 196.7,127.8 232.9,118.9 269.1,109.7 305.3,96.6 341.5,88.2 377.6,79.8 413.8,68.7 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,157.2 124.4,142.1 160.5,135.5 196.7,125.0 232.9,121.5 269.1,109.9 305.3,95.5 341.5,92.1 377.6,74.9 413.8,63.4 450.0,59.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,160.1 124.4,142.1 160.5,144.1 196.7,125.1 232.9,121.5 269.1,102.9 305.3,96.6 341.5,92.0 377.6,83.5 413.8,72.9 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-add-0">add@base</label></div></figure></div>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `div@base` D18 | 6.71 ns | 6.25 ns | 5.53 ns | 8.27 ns | 8.92 ns |
| `div@base` D38 | 12.7 ns | 11.6 ns | 15.6 ns | 52.9 ns | 56.6 ns |
| `div@base` D57 | 23.4 ns | 26.8 ns | 63.5 ns | 111 ns | 113 ns |
| `div@base` D76 | 25.7 ns | 59.1 ns | 53.7 ns | 114 ns | 124 ns |
| `div@base` D115 | 33.5 ns | 83.9 ns | 104 ns | 201 ns | 253 ns |
| `div@base` D153 | 36.4 ns | 110 ns | 155 ns | 250 ns | 319 ns |
| `div@base` D230 | 81.6 ns | 171 ns | 149 ns | 269 ns | 570 ns |
| `div@base` D307 | 91.9 ns | 221 ns | 363 ns | 651 ns | 890 ns |
| `div@base` D462 | 217 ns | 430 ns | 587 ns | 1.02 µs | 1.46 µs |
| `div@base` D616 | 225 ns | 649 ns | 870 ns | 1.55 µs | 2.07 µs |
| `div@base` D924 | 385 ns | 1.09 µs | 1.5 µs | 2.87 µs | 4.51 µs |
| `div@base` D1232 | 505 ns | 1.75 µs | 3.48 µs | 3.98 µs | 7.8 µs |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-div-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,168.7 88.2,154.9 124.4,141.5 160.5,139.5 196.7,133.7 232.9,132.0 269.1,114.4 305.3,111.8 341.5,93.2 377.6,92.4 413.8,80.7 450.0,74.8 450.0,15.4 413.8,27.3 377.6,44.2 341.5,51.8 305.3,62.5 269.1,72.2 232.9,84.8 196.7,89.8 160.5,105.4 124.4,107.4 88.2,122.4 52.0,162.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,168.7 88.2,154.9 124.4,141.5 160.5,139.5 196.7,133.7 232.9,132.0 269.1,114.4 305.3,111.8 341.5,93.2 377.6,92.4 413.8,80.7 450.0,74.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.2 88.2,156.7 124.4,138.6 160.5,121.4 196.7,113.8 232.9,108.0 269.1,98.4 305.3,92.8 341.5,78.3 377.6,69.4 413.8,58.1 450.0,47.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.9 88.2,150.3 124.4,119.9 160.5,123.5 196.7,109.1 232.9,100.4 269.1,101.4 305.3,82.0 341.5,71.6 377.6,63.0 413.8,51.2 450.0,32.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.1 88.2,123.8 124.4,107.8 160.5,107.2 196.7,94.9 232.9,90.1 269.1,88.5 305.3,69.3 341.5,59.5 377.6,50.5 413.8,37.1 450.0,30.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.5 88.2,122.4 124.4,107.4 160.5,105.4 196.7,89.8 232.9,84.8 269.1,72.2 305.3,62.5 341.5,51.8 377.6,44.2 413.8,27.3 450.0,15.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-div-0">div@base</label></div></figure></div>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `mul@base` D18 | 0.692 ns | 2.81 ns | 3.03 ns | 4.93 ns | 4.97 ns |
| `mul@base` D38 | 3.94 ns | 14.2 ns | 24.7 ns | 20.9 ns | 18.3 ns |
| `mul@base` D57 | 6.54 ns | 16.8 ns | 33.6 ns | 74.2 ns | 75.1 ns |
| `mul@base` D76 | 5.65 ns | 34.4 ns | 32.2 ns | 82 ns | 85.7 ns |
| `mul@base` D115 | 12.9 ns | 50 ns | 82.4 ns | 211 ns | 250 ns |
| `mul@base` D153 | 11.3 ns | 53.9 ns | 122 ns | 236 ns | 357 ns |
| `mul@base` D230 | 27.8 ns | 134 ns | 222 ns | 344 ns | 1 µs |
| `mul@base` D307 | 33.8 ns | 167 ns | 463 ns | 1.08 µs | 1.39 µs |
| `mul@base` D462 | 95.2 ns | 420 ns | 1.05 µs | 1.66 µs | 2.38 µs |
| `mul@base` D616 | 96.4 ns | 706 ns | 1.31 µs | 2.3 µs | 3.43 µs |
| `mul@base` D924 | 162 ns | 1.48 µs | 2.26 µs | 5.43 µs | 7.6 µs |
| `mul@base` D1232 | 182 ns | 2.2 µs | 4.71 µs | 8.41 µs | 14.1 µs |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-mul-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,182.0 88.2,156.8 124.4,149.5 160.5,151.6 196.7,139.6 232.9,141.6 269.1,128.5 305.3,125.7 341.5,110.7 377.6,110.5 413.8,103.0 450.0,101.3 450.0,38.3 413.8,47.3 377.6,58.8 341.5,64.1 305.3,71.9 269.1,76.6 232.9,91.6 196.7,96.7 160.5,112.2 124.4,114.1 88.2,134.6 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.0 88.2,156.8 124.4,149.5 160.5,151.6 196.7,139.6 232.9,141.6 269.1,128.5 305.3,125.7 341.5,110.7 377.6,110.5 413.8,103.0 450.0,101.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.7 88.2,138.3 124.4,135.8 160.5,125.5 196.7,120.0 232.9,118.9 269.1,105.7 305.3,102.6 341.5,89.2 377.6,81.7 413.8,71.0 450.0,65.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.6 88.2,130.2 124.4,125.8 160.5,126.4 196.7,112.8 232.9,107.1 269.1,98.4 305.3,87.8 341.5,76.0 377.6,72.8 413.8,64.9 450.0,54.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.6 88.2,132.7 124.4,114.3 160.5,112.9 196.7,99.2 232.9,97.5 269.1,92.1 305.3,75.6 341.5,69.3 377.6,64.6 413.8,52.2 450.0,45.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,134.6 124.4,114.1 160.5,112.2 196.7,96.7 232.9,91.6 269.1,76.6 305.3,71.9 341.5,64.1 377.6,58.8 413.8,47.3 450.0,38.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-mul-0">mul@base</label></div></figure></div>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `neg@base` D18 | 0.346 ns | 0.346 ns | 0.622 ns | 0.622 ns | 0.703 ns |
| `neg@base` D38 | 1.45 ns | 1.33 ns | 1.15 ns | 1.08 ns | 0.941 ns |
| `neg@base` D57 | 1.68 ns | 1.35 ns | 1.57 ns | 1.69 ns | 1.68 ns |
| `neg@base` D76 | 2.16 ns | 2.09 ns | 1.51 ns | 2.1 ns | 2.02 ns |
| `neg@base` D115 | 2.67 ns | 2.84 ns | 2.83 ns | 3.55 ns | 3.55 ns |
| `neg@base` D153 | 2.72 ns | 3.79 ns | 4.6 ns | 4.29 ns | 4.29 ns |
| `neg@base` D230 | 5.99 ns | 6.65 ns | 4.01 ns | 4.03 ns | 7.17 ns |
| `neg@base` D307 | 7.11 ns | 11.1 ns | 11.1 ns | 12.4 ns | 11.1 ns |
| `neg@base` D462 | 18.3 ns | 15 ns | 13.9 ns | 10.9 ns | 10.9 ns |
| `neg@base` D616 | 18 ns | 21.9 ns | 18.6 ns | 17.9 ns | 15.5 ns |
| `neg@base` D924 | 63.1 ns | 76.4 ns | 64.1 ns | 84.9 ns | 58.6 ns |
| `neg@base` D1232 | 47 ns | 61.5 ns | 61.4 ns | 65.3 ns | 69.8 ns |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-neg-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,174.1 88.2,132.6 124.4,128.3 160.5,121.0 196.7,114.9 232.9,114.4 269.1,91.5 305.3,86.5 341.5,59.1 377.6,59.6 413.8,23.3 450.0,31.8 450.0,20.4 413.8,25.5 377.6,64.1 341.5,74.1 305.3,73.7 269.1,86.3 232.9,101.1 196.7,106.7 160.5,122.9 124.4,128.3 88.2,145.1 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.1 88.2,132.6 124.4,128.3 160.5,121.0 196.7,114.9 232.9,114.4 269.1,91.5 305.3,86.5 341.5,59.1 377.6,59.6 413.8,23.3 450.0,31.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,174.1 88.2,135.2 124.4,134.7 160.5,122.0 196.7,113.2 232.9,104.8 269.1,88.5 305.3,73.7 341.5,64.9 377.6,53.9 413.8,17.8 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,139.2 124.4,130.2 160.5,131.5 196.7,113.2 232.9,99.1 269.1,103.1 305.3,73.7 341.5,67.1 377.6,58.6 413.8,22.9 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,141.0 124.4,128.2 160.5,121.9 196.7,106.6 232.9,101.1 269.1,103.0 305.3,70.5 341.5,74.1 377.6,59.8 413.8,14.7 450.0,22.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,145.1 124.4,128.3 160.5,122.9 196.7,106.7 232.9,101.1 269.1,86.3 305.3,73.7 341.5,74.1 377.6,64.1 413.8,25.5 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-neg-0">neg@base</label></div></figure></div>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `rem@base` D18 | 1.73 ns | 1.73 ns | 1.87 ns | 2.18 ns | 2.46 ns |
| `rem@base` D38 | 3.55 ns | 4.26 ns | 3.58 ns | 2.18 ns | 1.84 ns |
| `rem@base` D57 | 7.17 ns | 6.28 ns | 5.51 ns | 7.17 ns | 7.16 ns |
| `rem@base` D76 | 9.54 ns | 8.43 ns | 5.14 ns | 8.61 ns | 5.55 ns |
| `rem@base` D115 | 10 ns | 12.8 ns | 12.8 ns | 14.4 ns | 14.4 ns |
| `rem@base` D153 | 9.92 ns | 16.1 ns | 20 ns | 21.5 ns | 17.1 ns |
| `rem@base` D230 | 32.3 ns | 36 ns | 16.8 ns | 16.6 ns | 32.1 ns |
| `rem@base` D307 | 31.8 ns | 43.8 ns | 43 ns | 48 ns | 42.6 ns |
| `rem@base` D462 | 90.6 ns | 73.5 ns | 67.9 ns | 53.7 ns | 51.8 ns |
| `rem@base` D616 | 77.9 ns | 98.3 ns | 63.4 ns | 72 ns | 70.1 ns |
| `rem@base` D924 | 108 ns | 98.8 ns | 95.5 ns | 106 ns | 52.9 ns |
| `rem@base` D1232 | 133 ns | 133 ns | 130 ns | 124 ns | 122 ns |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-rem-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,194.1 88.2,173.3 124.4,153.0 160.5,144.7 196.7,143.2 232.9,143.6 269.1,109.3 305.3,109.9 341.5,79.5 377.6,83.9 413.8,74.4 450.0,68.4 450.0,70.9 413.8,95.1 377.6,86.9 341.5,95.7 305.3,101.4 269.1,109.6 232.9,127.9 196.7,132.8 160.5,160.4 124.4,153.0 88.2,192.3 52.0,183.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.1 88.2,173.3 124.4,153.0 160.5,144.7 196.7,143.2 232.9,143.6 269.1,109.3 305.3,109.9 341.5,79.5 377.6,83.9 413.8,74.4 450.0,68.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,194.1 88.2,168.1 124.4,156.8 160.5,148.3 196.7,136.3 232.9,129.6 269.1,106.2 305.3,100.6 341.5,85.6 377.6,77.2 413.8,77.0 450.0,68.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,173.1 124.4,160.6 160.5,162.6 196.7,136.3 232.9,123.2 269.1,128.4 305.3,101.1 341.5,87.9 377.6,89.8 413.8,78.0 450.0,69.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,187.4 124.4,153.0 160.5,147.7 196.7,132.8 232.9,121.1 269.1,128.7 305.3,97.9 341.5,94.7 377.6,86.2 413.8,75.1 450.0,70.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,192.3 124.4,153.0 160.5,160.4 196.7,132.8 232.9,127.9 269.1,109.6 305.3,101.4 341.5,95.7 377.6,86.9 413.8,95.1 450.0,70.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-rem-0">rem@base</label></div></figure></div>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `sub@base` D18 | 1.05 ns | 1.05 ns | 0.935 ns | 0.935 ns | 1.05 ns |
| `sub@base` D38 | 1.81 ns | 1.61 ns | 1.44 ns | 1.04 ns | 0.908 ns |
| `sub@base` D57 | 2.26 ns | 1.94 ns | 2.02 ns | 2.25 ns | 2.25 ns |
| `sub@base` D76 | 3.45 ns | 3.08 ns | 2.19 ns | 3.09 ns | 2.42 ns |
| `sub@base` D115 | 4.51 ns | 4.9 ns | 4.88 ns | 5.58 ns | 5.55 ns |
| `sub@base` D153 | 5.12 ns | 7.55 ns | 8.45 ns | 7.57 ns | 7.58 ns |
| `sub@base` D230 | 16.2 ns | 17.7 ns | 11.1 ns | 10.9 ns | 16.2 ns |
| `sub@base` D307 | 18.1 ns | 23.5 ns | 23.5 ns | 25.2 ns | 23.5 ns |
| `sub@base` D462 | 40.7 ns | 37.1 ns | 33.6 ns | 29.6 ns | 29.6 ns |
| `sub@base` D616 | 49.8 ns | 57.4 ns | 49.1 ns | 50.1 ns | 34 ns |
| `sub@base` D924 | 84.9 ns | 76.3 ns | 74.1 ns | 84.6 ns | 60.8 ns |
| `sub@base` D1232 | 95.2 ns | 95.6 ns | 95.5 ns | 104 ns | 106 ns |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-sub-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,159.0 88.2,147.1 124.4,142.3 160.5,133.1 196.7,127.3 232.9,124.5 269.1,99.6 305.3,97.1 341.5,79.5 377.6,75.1 413.8,63.5 450.0,61.1 450.0,58.7 413.8,70.8 377.6,83.4 341.5,86.4 305.3,91.5 269.1,99.5 232.9,116.0 196.7,122.8 160.5,140.8 124.4,142.4 88.2,162.1 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,159.0 88.2,147.1 124.4,142.3 160.5,133.1 196.7,127.3 232.9,124.5 269.1,99.6 305.3,97.1 341.5,79.5 377.6,75.1 413.8,63.5 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,159.0 88.2,149.6 124.4,145.6 160.5,135.6 196.7,125.5 232.9,116.1 269.1,97.7 305.3,91.5 341.5,81.5 377.6,72.0 413.8,65.9 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,152.1 124.4,144.7 160.5,143.0 196.7,125.6 232.9,113.7 269.1,107.7 305.3,91.5 341.5,83.7 377.6,75.5 413.8,66.5 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,159.0 124.4,142.3 160.5,135.5 196.7,122.7 232.9,116.0 269.1,108.1 305.3,89.9 341.5,86.5 377.6,75.0 413.8,63.6 450.0,59.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,162.1 124.4,142.4 160.5,140.8 196.7,122.8 232.9,116.0 269.1,99.5 305.3,91.5 341.5,86.4 377.6,83.4 413.8,70.8 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-sub-0">sub@base</label></div></figure></div>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
