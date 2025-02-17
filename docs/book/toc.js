// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded affix "><a href="installation.html">Installation</a></li><li class="chapter-item expanded affix "><a href="gettingstart.html">Getting Start</a></li><li class="chapter-item expanded "><a href="chapter_1/index.html"><strong aria-hidden="true">1.</strong> Styling</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="chapter_1/how_to_write_styles.html"><strong aria-hidden="true">1.1.</strong> How to write styles in json file?</a></li></ol></li><li class="chapter-item expanded "><a href="chapter_2/index.html"><strong aria-hidden="true">2.</strong> Interaction</a></li><li class="chapter-item expanded "><a href="chapter_3/index.html"><strong aria-hidden="true">3.</strong> Widgets</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="chapter_3/container.html"><strong aria-hidden="true">3.1.</strong> Container</a></li><li class="chapter-item expanded "><a href="chapter_3/button.html"><strong aria-hidden="true">3.2.</strong> Button</a></li><li class="chapter-item expanded "><a href="chapter_3/text.html"><strong aria-hidden="true">3.3.</strong> Text</a></li><li class="chapter-item expanded "><a href="chapter_3/fps_text.html"><strong aria-hidden="true">3.4.</strong> FpsText</a></li><li class="chapter-item expanded "><a href="chapter_3/text_input.html"><strong aria-hidden="true">3.5.</strong> TextInput</a></li><li class="chapter-item expanded "><a href="chapter_3/selection.html"><strong aria-hidden="true">3.6.</strong> Selection</a></li><li class="chapter-item expanded "><a href="chapter_3/circular.html"><strong aria-hidden="true">3.7.</strong> Circular</a></li><li class="chapter-item expanded "><a href="chapter_3/modal.html"><strong aria-hidden="true">3.8.</strong> Modal</a></li><li class="chapter-item expanded "><a href="chapter_3/list_view.html"><strong aria-hidden="true">3.9.</strong> ListView</a></li><li class="chapter-item expanded "><a href="chapter_3/image.html"><strong aria-hidden="true">3.10.</strong> Image</a></li><li class="chapter-item expanded "><a href="chapter_3/bg_image.html"><strong aria-hidden="true">3.11.</strong> Background Image</a></li><li class="chapter-item expanded "><a href="chapter_3/progress_bar.html"><strong aria-hidden="true">3.12.</strong> Progress Bar</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
