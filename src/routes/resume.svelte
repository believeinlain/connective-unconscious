<script>
  import Page from '$lib/layout/Page.svelte';
  import Entry from '$lib/resume/Entry.svelte';
</script>

<Page page_title="Stephanie Aelmore">
  <span>
    <p>
      <a href="https://linkedin.com/in/stephanie-aelmore">
        https://linkedin.com/in/stephanie-aelmore
      </a>
      -
      <a href="https://github.com/believeinlain">
        https://github.com/believeinlain
      </a>
    </p>
    <h3>Work Experience</h3>
    <Entry>
      <span slot="position">
        Computer Engineer at Oceanit Laboratories Inc.
      </span>
      <span slot="dates">06/2022 - Present</span>
      <span slot="bullet-list">
        <li>
          Developed Forced Execution engine in Python leading to >150% better
          code coverage
        </li>
      </span>
    </Entry>
    <Entry>
      <span slot="position">
        Electrical Engineering Teaching Assistant at University of Hawaiʻi at
        Mānoa
      </span>
      <span slot="dates">01/2022 - 05/2022</span>
      <span slot="bullet-list">
        <li>
          Led 2 sections of Basic Circuit Analysis II laboratory, supervising
          and tutoring 38 students
        </li>
        <li>
          Graded lab reports, assisted students with circuit troubleshooting,
          performed part requisitions
        </li>
      </span>
    </Entry>
    <Entry>
      <span slot="position">
        Assistant Researcher at Naval Information Warfare Center Pacific
      </span>
      <span slot="dates">01/2020 - 08/2021</span>
      <span slot="bullet-list">
        <li>
          Conducted self-directed research at a graduate level, culminating in a
          publication and invention disclosure
        </li>
        <li>
          Evaluated, developed, and implemented event-based computer vision
          algorithms (Arc*, SOFAS) in Python
        </li>
        <li>
          Designed, performed, and analyzed experiments functionalizing
          semiconductor quantum dots and ionogels
        </li>
        <li>
          Utilized MATLAB to collect and analyze data and provided routine
          technical reports to project PI
        </li>
      </span>
    </Entry>
    <Entry>
      <span slot="position">
        Nuclear Electronic Technician First Class at United States Navy
      </span>
      <span slot="dates">10/2012 - 10/2018</span>
      <span slot="bullet-list">
        <li>
          12+ months as Lead Supervisor and Technical Consultant on reactor
          instrumentation and control systems, provided direct mentorship 10
          people resulting in team members receiving four promotions and three
          performance-based awards
        </li>
        <li>
          Led department-wide classroom training on reactor theory and
          instrumentation, resulting in the department achieving top scores
          three years in a row on operational examinations
        </li>
      </span>
    </Entry>
    <h3>Education</h3>
    <Entry>
      <span slot="position">
        Bachelor of Science in Computer Engineering, Cum Laude
      </span>
      <span slot="location"> University of Hawaiʻi at Mānoa </span>
      <span slot="dates">01/2019 - 05/2022</span>
      <span slot="gpa">GPA: 3.88</span>
      <span slot="bullet-list">
        <li>
          Chapter Treasurer for Delta Omega Chapter of IEEE-HKN Honor Society,
          2021-2022
          <ul>
            <li>
              Applied for and received funding, handled dues and reimbursements
              with zero incidents
            </li>
          </ul>
        </li>
        <li>
          Software Engineer for VIP Team RoSE, 2021-2022
          <ul>
            <li>
              Designed software architecture, selected hardware and software
              components, and programmed UART serial communications for onboard
              computing on mock mars rover for the University Rover Competition
            </li>
            <li>
              Created development and test plan using publisher-subscriber
              pattern and continuous integration
            </li>
          </ul>
        </li>
      </span>
    </Entry>
    <h3>Projects</h3>
    <Entry>
      <span slot="position">
        Real-Time Event-Based Tracking and Detection for Maritime Environments
      </span>
      <span slot="location"> Naval Information Warfare Center Pacific </span>
      <span slot="dates">10/2020 - 08/2021</span>
      <span slot="bullet-list">
        <li>
          Designed, implemented, and evaluated novel event-based tracking and
          clustering algorithm in Python and C++
        </li>
        <li>
          Optimized using iterative development process, resulting in
          multi-threaded C++ core wrapped in Cython
        </li>
        <li>
          Published as first author in IEEE AIPR 2021 Proceedings - preprint
          available at
          <a href="https://doi.org/10.1109/AIPR52630.2021.9762225">
            https://doi.org/10.1109/AIPR52630.2021.9762225
          </a>
        </li>
      </span>
    </Entry>
    <Entry>
      <span slot="position"> Quantum Dot Photoemitter - QDPi </span>
      <span slot="location"> Naval Information Warfare Center Pacific </span>
      <span slot="dates">01/2020 - 09/2020</span>
      <span slot="bullet-list">
        <li>
          Developed test routines in MATLAB using Object-Oriented Programming to
          run experiments and analyze data
        </li>
        <li>
          Utilized byte-wise USB serial communications to interface with
          spectrometer and parse binary data in MATLAB
        </li>
        <li>
          Designed and executed laboratory experiments to refine ionogel
          chemical synthesis techniques
        </li>
      </span>
    </Entry>
  </span>
  <span slot="footer" />
</Page>

<style>
  p,
  h3 {
    text-align: center;
  }
  h3 {
    font-weight: normal;
    text-decoration: underline;
    line-height: 50%;
    text-transform: uppercase;
  }
</style>
