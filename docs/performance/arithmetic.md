# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.06 ns | 0.938 ns | 0.937 ns | 0.936 ns | 0.939 ns |
| D38 | 1.82 ns | 1.61 ns | 1.82 ns | 0.981 ns | 1.81 ns |
| D57 | 2.29 ns | 1.28 ns | 2.29 ns | 2.28 ns | 1.55 ns |
| D76 | 2.08 ns | 3.08 ns | 1.78 ns | 3.49 ns | 3.48 ns |
| D115 | 4.41 ns | 4.39 ns | 3.88 ns | 4.42 ns | 4.38 ns |
| D153 | 6.63 ns | 3.69 ns | 6.64 ns | 5.91 ns | 4.5 ns |
| D230 | 12.2 ns | 15.4 ns | 15.3 ns | 15.4 ns | 13.8 ns |
| D307 | 18.6 ns | 18.5 ns | 18.6 ns | 18.6 ns | 19.6 ns |
| D462 | 33.4 ns | 41.2 ns | 29.6 ns | 32.5 ns | 33.3 ns |
| D616 | 34.1 ns | 45.4 ns | 49.9 ns | 54.3 ns | 32.1 ns |
| D924 | 74.7 ns | 52.6 ns | 85.1 ns | 69.9 ns | 85.3 ns |
| D1232 | 108 ns | 106 ns | 95.1 ns | 95 ns | 106 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,147.0 124.4,142.0 160.5,144.1 196.7,127.8 232.9,118.9 269.1,105.7 305.3,96.5 341.5,83.8 377.6,83.3 413.8,66.3 450.0,58.4 450.0,58.8 413.8,63.5 377.6,84.7 341.5,83.9 305.3,95.4 269.1,102.9 232.9,127.3 196.7,127.9 160.5,132.9 124.4,150.5 88.2,147.2 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,147.0 124.4,142.0 160.5,144.1 196.7,127.8 232.9,118.9 269.1,105.7 305.3,96.5 341.5,83.8 377.6,83.3 413.8,66.3 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,149.7 124.4,154.7 160.5,135.6 196.7,127.9 232.9,131.6 269.1,100.7 305.3,96.6 341.5,79.3 377.6,77.2 413.8,74.0 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,146.9 124.4,142.1 160.5,147.4 196.7,130.6 232.9,118.9 269.1,100.8 305.3,96.5 341.5,86.5 377.6,75.1 413.8,63.5 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,160.4 124.4,142.1 160.5,132.8 196.7,127.7 232.9,121.4 269.1,100.6 305.3,96.6 341.5,84.4 377.6,73.3 413.8,67.8 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,147.2 124.4,150.5 160.5,132.9 196.7,127.9 232.9,127.3 269.1,102.9 305.3,95.4 341.5,83.9 377.6,84.7 413.8,63.5 450.0,58.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.45 ns | 5.44 ns | 5.6 ns | 8.32 ns | 7.83 ns |
| D38 | 12.7 ns | 11.6 ns | 14.3 ns | 60.4 ns | 66.9 ns |
| D57 | 38 ns | 28.3 ns | 69 ns | 107 ns | 86.1 ns |
| D76 | 31.3 ns | 61.4 ns | 44.2 ns | 110 ns | 145 ns |
| D115 | 56.4 ns | 83.1 ns | 87.1 ns | 177 ns | 219 ns |
| D153 | 69.2 ns | 65.5 ns | 153 ns | 232 ns | 318 ns |
| D230 | 83.6 ns | 168 ns | 251 ns | 426 ns | 562 ns |
| D307 | 132 ns | 227 ns | 362 ns | 572 ns | 952 ns |
| D462 | 234 ns | 413 ns | 685 ns | 1.16 µs | 1.46 µs |
| D616 | 176 ns | 613 ns | 1.03 µs | 1.83 µs | 1.35 µs |
| D924 | 389 ns | 689 ns | 2.3 µs | 2.48 µs | 4.68 µs |
| D1232 | 586 ns | 1.97 µs | 3.48 µs | 4.37 µs | 7.98 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,169.5 88.2,154.7 124.4,131.0 160.5,135.2 196.7,122.4 232.9,118.0 269.1,113.9 305.3,104.0 341.5,91.5 377.6,97.7 413.8,80.5 450.0,71.6 450.0,14.9 413.8,26.5 377.6,53.5 341.5,51.7 305.3,61.1 269.1,72.5 232.9,84.9 196.7,93.0 160.5,101.9 124.4,113.3 88.2,118.7 52.0,165.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,169.5 88.2,154.7 124.4,131.0 160.5,135.2 196.7,122.4 232.9,118.0 269.1,113.9 305.3,104.0 341.5,91.5 377.6,97.7 413.8,80.5 450.0,71.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,173.2 88.2,156.7 124.4,137.4 160.5,120.6 196.7,114.0 232.9,119.2 269.1,98.8 305.3,92.2 341.5,79.2 377.6,70.6 413.8,68.1 450.0,45.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.6 88.2,152.2 124.4,118.1 160.5,127.7 196.7,113.0 232.9,100.7 269.1,90.1 305.3,82.1 341.5,68.2 377.6,59.3 413.8,41.9 450.0,32.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.0 88.2,121.0 124.4,108.6 160.5,108.0 196.7,97.6 232.9,91.7 269.1,78.5 305.3,72.1 341.5,56.8 377.6,46.9 413.8,40.3 450.0,28.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.3 88.2,118.7 124.4,113.3 160.5,101.9 196.7,93.0 232.9,84.9 269.1,72.5 305.3,61.1 341.5,51.7 377.6,53.5 413.8,26.5 450.0,14.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 2.87 ns | 3.13 ns | 4.93 ns | 5.11 ns |
| D38 | 3.94 ns | 13.9 ns | 30.3 ns | 18 ns | 36.4 ns |
| D57 | 6.23 ns | 12.9 ns | 33.4 ns | 71.7 ns | 55.6 ns |
| D76 | 5.21 ns | 35 ns | 26.5 ns | 83.8 ns | 109 ns |
| D115 | 13.2 ns | 47.6 ns | 72.3 ns | 197 ns | 229 ns |
| D153 | 16.8 ns | 29.1 ns | 120 ns | 242 ns | 336 ns |
| D230 | 24.4 ns | 127 ns | 368 ns | 567 ns | 995 ns |
| D307 | 44.9 ns | 166 ns | 453 ns | 1.02 µs | 1.44 µs |
| D462 | 87.1 ns | 408 ns | 1.24 µs | 1.83 µs | 2.62 µs |
| D616 | 59.4 ns | 653 ns | 1.72 µs | 2.68 µs | 2.14 µs |
| D924 | 137 ns | 923 ns | 3.17 µs | 4.48 µs | 8.2 µs |
| D1232 | 203 ns | 2.37 µs | 4.57 µs | 8.18 µs | 14 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,209.1 88.2,186.2 124.4,178.2 160.5,181.3 196.7,165.2 232.9,161.0 269.1,154.5 305.3,143.9 341.5,132.4 377.6,139.0 413.8,124.5 450.0,117.7 450.0,44.2 413.8,53.5 377.6,76.8 341.5,73.2 305.3,83.6 269.1,90.1 232.9,108.9 196.7,115.6 160.5,128.6 124.4,140.2 88.2,147.6 52.0,181.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,209.1 88.2,186.2 124.4,178.2 160.5,181.3 196.7,165.2 232.9,161.0 269.1,154.5 305.3,143.9 341.5,132.4 377.6,139.0 413.8,124.5 450.0,117.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.7 88.2,164.3 124.4,165.6 160.5,148.3 196.7,142.9 232.9,151.4 269.1,125.8 305.3,121.2 341.5,105.6 377.6,97.4 413.8,91.4 450.0,75.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.2 88.2,150.8 124.4,149.1 160.5,153.1 196.7,135.6 232.9,126.8 269.1,107.4 305.3,103.8 341.5,86.3 377.6,80.5 413.8,69.9 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.3 88.2,159.8 124.4,135.8 160.5,133.1 196.7,118.2 232.9,114.7 269.1,99.9 305.3,89.7 341.5,79.5 377.6,72.9 413.8,64.0 450.0,53.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.7 88.2,147.6 124.4,140.2 160.5,128.6 196.7,115.6 232.9,108.9 269.1,90.1 305.3,83.6 341.5,73.2 377.6,76.8 413.8,53.5 450.0,44.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.703 ns | 0.622 ns | 0.622 ns | 0.622 ns | 0.622 ns |
| D38 | 1.45 ns | 1.32 ns | 1.45 ns | 0.932 ns | 1.45 ns |
| D57 | 1.87 ns | 1.05 ns | 1.87 ns | 1.87 ns | 1.33 ns |
| D76 | 1.68 ns | 2.18 ns | 1.45 ns | 2.17 ns | 2.63 ns |
| D115 | 2.85 ns | 2.85 ns | 2.46 ns | 3.29 ns | 3.28 ns |
| D153 | 4.22 ns | 2.37 ns | 4.6 ns | 4.49 ns | 3.48 ns |
| D230 | 4.29 ns | 6.65 ns | 7.24 ns | 7.24 ns | 7.17 ns |
| D307 | 10.9 ns | 11.1 ns | 11.1 ns | 11.1 ns | 12.5 ns |
| D462 | 17.5 ns | 14.9 ns | 15.3 ns | 17.6 ns | 17 ns |
| D616 | 14.5 ns | 20 ns | 20 ns | 19.8 ns | 13.2 ns |
| D924 | 55 ns | 47.5 ns | 84.8 ns | 76.5 ns | 84.6 ns |
| D1232 | 54.4 ns | 69.9 ns | 61.4 ns | 61.4 ns | 69.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.5 88.2,132.6 124.4,125.2 160.5,128.3 196.7,113.0 232.9,101.7 269.1,101.1 305.3,74.2 341.5,60.4 377.6,65.9 413.8,27.3 450.0,27.6 450.0,20.4 413.8,14.8 377.6,68.7 341.5,61.3 305.3,70.2 269.1,86.3 232.9,107.2 196.7,108.9 160.5,115.4 124.4,135.1 88.2,132.7 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.5 88.2,132.6 124.4,125.2 160.5,128.3 196.7,113.0 232.9,101.7 269.1,101.1 305.3,74.2 341.5,60.4 377.6,65.9 413.8,27.3 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,135.2 124.4,142.0 160.5,120.7 196.7,113.0 232.9,118.3 269.1,88.5 305.3,73.7 341.5,65.1 377.6,56.6 413.8,31.6 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.6 124.4,125.2 160.5,132.7 196.7,117.3 232.9,99.1 269.1,86.0 305.3,73.7 341.5,64.3 377.6,56.6 413.8,14.8 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,145.4 124.4,125.2 160.5,121.0 196.7,108.9 232.9,99.9 269.1,86.0 305.3,73.7 341.5,60.2 377.6,57.0 413.8,17.7 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.7 124.4,135.1 160.5,115.4 196.7,108.9 232.9,107.2 269.1,86.3 305.3,70.2 341.5,61.3 377.6,68.7 413.8,14.8 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.11 ns | 1.87 ns | 1.87 ns | 2.18 ns | 2.18 ns |
| D38 | 8.58 ns | 7.15 ns | 14.5 ns | 6.8 ns | 16.3 ns |
| D57 | 7.16 ns | 3.7 ns | 7.17 ns | 7.17 ns | 3.85 ns |
| D76 | 6.09 ns | 8.52 ns | 4.51 ns | 9.75 ns | 9.83 ns |
| D115 | 12.7 ns | 12.4 ns | 10.9 ns | 12.4 ns | 12.4 ns |
| D153 | 20.7 ns | 9.21 ns | 20.1 ns | 16.3 ns | 11.9 ns |
| D230 | 24.9 ns | 38.2 ns | 36 ns | 36.4 ns | 31.8 ns |
| D307 | 41.9 ns | 43.2 ns | 43.4 ns | 42.7 ns | 47.8 ns |
| D462 | 90 ns | 81.2 ns | 77 ns | 84.5 ns | 83.2 ns |
| D616 | 58.7 ns | 81.9 ns | 84.2 ns | 78.5 ns | 38.5 ns |
| D924 | 112 ns | 67.2 ns | 110 ns | 78.6 ns | 119 ns |
| D1232 | 143 ns | 137 ns | 119 ns | 120 ns | 125 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.4 88.2,147.8 124.4,153.0 160.5,157.7 196.7,136.3 232.9,122.2 269.1,116.9 305.3,101.9 341.5,79.7 377.6,92.1 413.8,73.5 450.0,66.2 450.0,70.2 413.8,71.5 377.6,104.3 341.5,82.0 305.3,98.0 269.1,109.8 232.9,138.3 196.7,137.0 160.5,143.8 124.4,171.0 88.2,129.1 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.4 88.2,147.8 124.4,153.0 160.5,157.7 196.7,136.3 232.9,122.2 269.1,116.9 305.3,101.9 341.5,79.7 377.6,92.1 413.8,73.5 450.0,66.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,153.0 124.4,172.1 160.5,148.0 196.7,137.1 232.9,145.7 269.1,104.5 305.3,101.0 341.5,82.7 377.6,82.5 413.8,88.2 450.0,67.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,132.6 124.4,153.0 160.5,166.4 196.7,140.8 232.9,123.2 269.1,106.3 305.3,100.8 341.5,84.2 377.6,81.6 413.8,74.0 450.0,71.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,154.5 124.4,153.0 160.5,144.1 196.7,137.0 232.9,129.3 269.1,106.0 305.3,101.3 341.5,81.5 377.6,83.7 413.8,83.6 450.0,71.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,129.1 124.4,171.0 160.5,143.8 196.7,137.0 232.9,138.3 269.1,109.8 305.3,98.0 341.5,82.0 377.6,104.3 413.8,71.5 450.0,70.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.06 ns | 0.935 ns | 0.937 ns | 0.935 ns | 0.935 ns |
| D38 | 1.81 ns | 1.6 ns | 1.82 ns | 0.898 ns | 1.83 ns |
| D57 | 2.28 ns | 1.3 ns | 2.27 ns | 2.27 ns | 1.66 ns |
| D76 | 2.44 ns | 3.12 ns | 2.07 ns | 3.45 ns | 3.47 ns |
| D115 | 4.84 ns | 4.84 ns | 4.3 ns | 4.83 ns | 4.85 ns |
| D153 | 8.47 ns | 4.79 ns | 8.47 ns | 7.65 ns | 6.37 ns |
| D230 | 13.7 ns | 17.8 ns | 17.6 ns | 17.6 ns | 16.2 ns |
| D307 | 23.3 ns | 23.4 ns | 23.3 ns | 23.4 ns | 25.2 ns |
| D462 | 43.1 ns | 49.4 ns | 37.2 ns | 40.5 ns | 43.3 ns |
| D616 | 37.5 ns | 45.8 ns | 50.5 ns | 55.1 ns | 31.7 ns |
| D924 | 75.7 ns | 56.8 ns | 84.8 ns | 76.7 ns | 84.9 ns |
| D1232 | 107 ns | 106 ns | 95.4 ns | 95.5 ns | 106 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,147.1 124.4,142.1 160.5,140.6 196.7,125.7 232.9,113.6 269.1,103.1 305.3,91.6 341.5,78.3 377.6,81.3 413.8,66.0 450.0,58.6 450.0,58.7 413.8,63.6 377.6,84.9 341.5,78.2 305.3,89.9 269.1,99.6 232.9,119.8 196.7,125.7 160.5,133.0 124.4,149.0 88.2,146.9 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,147.1 124.4,142.1 160.5,140.6 196.7,125.7 232.9,113.6 269.1,103.1 305.3,91.6 341.5,78.3 377.6,81.3 413.8,66.0 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.5 88.2,149.8 124.4,154.4 160.5,135.3 196.7,125.8 232.9,126.0 269.1,97.5 305.3,91.6 341.5,75.3 377.6,77.0 413.8,72.3 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,147.0 124.4,142.2 160.5,144.2 196.7,128.3 232.9,113.6 269.1,97.7 305.3,91.6 341.5,81.5 377.6,74.8 413.8,63.6 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,162.3 124.4,142.2 160.5,133.1 196.7,125.8 232.9,115.8 269.1,97.7 305.3,91.5 341.5,79.6 377.6,73.0 413.8,65.8 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,146.9 124.4,149.0 160.5,133.0 196.7,125.7 232.9,119.8 269.1,99.6 305.3,89.9 341.5,78.2 377.6,84.9 413.8,63.6 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
