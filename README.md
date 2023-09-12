<p align="center">
    <img width="120" src="docs/public/images/trident-logo.png"/>
    <br>
    <h1 align="center"> ShuLin - Ubuntu Hardening </h1>
</p>

## Problem Statement SIH1446

**Title:** Developing a GUI based hardening script for Ubuntu operating system with flexibility to cater for organizational security policies.

**Description:** Hardening of an operating system involves implementation of security measure to make the system compliant with the security policies of the organization. The procedure for hardening should be intuitive to allow ease of use by personnel with minimal IT skills. The goal of this problem statement is to generate a script which is undertakes hardening of Ubuntu OS using an GUI based approach. During the hardening process, the user should have the flexibility to make settings based on the organizations IT security policy provision like blocking ssh, usb, ToR etc. The grading of tool will be based on hardening functions implemented, attention to user experience and flexibility to take user settings. Developer should remember that security is of utmost importance.

**Organization:** National Technical Research Organization(NTRO)

## Naamkaran - ShuLin(शुलिन)

ShuLin originated from Sanskrit which means "The one who wields the trident" so we can use a trident in our designs and logo but i've tried and it doesn't look good though my artistic skills are questionable!

btw there is a Chinese district named Shulin :O

## What is NTRO

NTRO, the National Technical Research Organization, is India's specialized technical intelligence agency.

- Technical Intelligence Collection
- Satellite and Aerospace Surveillance
- Cybersecurity and Cyber Intelligence
- Electronic Signals Intelligence (ELINT)
- Communication Interception and Monitoring

**Organizational needs they might have:**
Since NTRO is an intelligence agency they will adhere to strict security policies and regulatory compliance, I mean they should! I have listed some of the possible needs.

- **Security Policies and Compliance:** They might have predefined security policies and compliance standards to safeguard national security. These encompass data security, access control, and information protection.

- **Customization for Security Policies:** Like `Venkatesh` suggested that in the external hackathon, they will suggest a lot of changes so we should design the architecture keep flexibility in mind to accommodate their(judges or NTRO's) unique security policies and guidelines.

- **Access Control and User Permissions:**

- **Audit and Logging:** Comprehensive auditing and logging are critical for monitoring and auditing system activities, serving as a history reference for a future intrusion!

- **Integration with Existing Tools:**

- **Usability and Training:** Proper training and documentation are essential to ensure effective use of the system hardening tools(ShuLin).

## Now what is hardening?

Since [Linux is not a secure OS](https://madaidans-insecurities.github.io/linux.html) we have to increase its security and that is called hardening.

System hardening is the process of securing a computer system by reducing its vulnerabilities and strengthening its defenses against potential threats and attacks. It involves implementing a series of security measures and configurations and reducing the attack surface.

## Some of the hardening measures listed by [Madaidan](https://madaidans-insecurities.github.io/guides/linux-hardening.html)

- **Kernel hardening:** Strengthening the Linux kernel's security by applying patches, reducing attack surfaces, and configuring security features like SELinux or AppArmor.
  - Kernel modules
  - change the Linux kernel i.e recompile it and install it on the cost of performance
  - <https://www.timesys.com/security/securing-your-linux-configuration-kernel-hardening/>
  - <https://www.kicksecure.com/wiki/Hardened-kernel>
  - kernel integrity subsystem

- **Mandatory access control:** MAC systems give fine-grained control over what programs can access. This means that your browser won't have access to your entire home directory or similar. (SELinux >>> Apparmor).

- **Sandboxing:** Isolating applications or processes from the rest of the system to limit potential damage if they are compromised.

- **Hardened memory allocator:** Using memory allocation techniques that reduce the risk of memory-related vulnerabilities like buffer overflows.

- **Hardened compilation flags:** Compiling software with security-focused compiler flags to reduce vulnerabilities and improve code robustness.

- **Memory-safe languages:** Choosing programs written in programming languages like Rust or Ada that inherently provide memory safety, reducing the risk of memory-related vulnerabilities.

- **The root account:** Restricting the use of the root (superuser) account to essential tasks to minimize the risk of accidental or malicious system changes.

- **Firewalls:** Configuring firewalls to control incoming and outgoing network traffic to protect against unauthorized access and threats. (Fail2safe - Venkatesh's suggestion)

- **Identifiers:** Ensuring unique user and group identifiers to control access permissions accurately.

- **File permissions:** Setting appropriate permissions on files and directories to restrict access to authorized users and groups.

- **Core dumps:** Controlling core dumps to prevent the exposure of sensitive information in the event of program crashes.

- **Swap:** Managing swap space securely to avoid exposing sensitive data.

- **PAM (Pluggable Authentication Module):** Implementing flexible authentication and password policies.

- **Microcode updates:** Keeping CPU microcode updated to address hardware vulnerabilities.

- **IPv6 privacy extensions:** Enabling privacy extensions for IPv6 addresses to enhance network security or we can disable IPv6 entirely if the organization needs it.

- **Partitioning and mount options:** Properly partitioning and configuring mount options to improve security and isolate data.

- **Entropy:**
  - **Additional entropy sources:** Incorporating additional sources of randomness to improve cryptographic security.
  - **RDRAND:** Using the RDRAND instruction for hardware-based random number generation.

- **Editing files as root:** Exercising caution when editing system configuration files as the root user to avoid unintentional changes.

- **Intrusion detection:** (Suricata, Snort) Security systems that monitor network traffic and system behavior in real-time to detect and respond to potential threats and attacks."

- **Distribution-specific hardening:** Implementing security measures specific to [Ubuntu](https://wiki.ubuntu.com/Security/).

- **Physical security:** Protecting physical access to servers and systems to prevent unauthorized tampering.

- **Best practices:** Adhering to established security best practices, including regular updates, strong password policies, and security awareness training. (Documentation and well-structured guidelines)

## Now what can we consider implementing for the internal hackathon!?

- File permissions
  - We can have a dashboard to toggle all the permissions and suggest the recommended configuration
  - We can create a file dialog from which the user can choose a folder/file to make it secure, by file dialog i mean frontend will send the directory path to the backend similar to what uploading files on the website does.
- Encryption of files/directories.
- Remove Apparmor and install SELinux

> configuring SELinux will be the hardest and tedious task

- Firewall(Fail2ban)
- Identifiers(User access management)
  - We can limit the access of users
  - We can check their passwords and if it is weak then ban them and notify them to change their password.
  - Create a stringent guideline for passwords.
- Intrusion detection(Suricata, Snort)
- System update and patching from GUI itself
- Disable booting from external devices

> we have to figure out how can we implement this, whether using BIOS or from the backend itself. If it requires tinkering with BIOS then we can flash a toast suggesting the user to change the setting.

- Closing hidden ports.

## Some insights from Venkatesh

> **Some clarifications:**
>
> We will have to prepare 2 ppts i.e one with official [SIH format](https://www.sih.gov.in/letters/Idea-Presentation-Format-SIH2023-College.pptx) and one for show casing our project in internal hackathon(there is no limit of slides or format)
>
> I'll refer to former as `SPOC ppt` and later as `Internal ppt`

- We can to include as much data as possible in those 2 slides of SPOC ppt

> "Jitna fek sakte ho utna feko utna do slides me"

- Try to include catchy words in those slides even if we won't include them in the final project 'cause no one will review your ppt after screening round
- SPOC ppt is just supposed to get us past the screening round
- Let's assume we have cleared the screening round then forget about PPTs from there onwards
- After clearing the screening round we will be invited to the `Nodal Center` for 36 hours hackathon in which we would need to create the project from scratch(theoretically) but no one does that and even judges know. So Judges will suggest so many major changes and they'll judge us based on how effectively we implement them!
- We need to figure out solutions to all the edge cases as judges ask very weird questions

> **Venkatesh's story time:**
>
> Their project was related to [KIOSK for Aadhar correction](https://cdn.discordapp.com/attachments/1150112553775677491/1150112604354781295/sih22.pdf)
>
> **Judge:** "Imagine bande ki ungli kat gyi toh fir biometric kaise karoge?"
> **Venkatesh:** We can use face biometric
> **Judge:** "Usko face bhi disfigure ho gya hai accident me fir kya karoge?"
>
> They cna ask questions similar to these, So we will need to pay extra attention to edge cases @Manav will be perfect for this job!

- So venkatesh's team faced a lot of problem staying awake for 36 hours straight
- About the GUI unhardening the system, so they have asked us to build the GUI plus its for the specific organizational need of NTRO workstations so that shouldn't be a problem.

## Let's come to the structure of SIH in detail

- First will be internal hackathon @13th September, 12:30 PM in E3-G14
  - We have direct competition from Ammar's team
  - There will be 35 Teams sent from Amity, Noida's SPOC of which 5 will be on the waiting list
- After we are selected in the internal hackathon, SIH will mail us the credentials for the SIH portal and we will need to upload the SPOC ppt over there in pdf format with a summary and other relevant documents(no one will read these "other relevant documents)
- After the selection in the screening round, we will be invited to the nodal center and the 36-hour hackathon will begin.
- The accommodation will be provided by SIH but the quality depends on the college i.e. Nodal center

> Apparently venkatesh's nodal center was chefs kiss

- In that 36 hours hackathon, we will need to implement changes suggested by judges upon invigilation
- There will be three rounds of invigilation i.e. one in the morning of day 1, then another in the night and a third will be in the afternoon of day 2
- All the teams competing for the same problem will be in the same room/lab so anything we discuss over there will be public

> **Story time:**
>
> Venkatesh ki team ke sath aisa hua tha, they had a feature of web portal in the first round of invigilation and jab tak third round aya vo sari teams ne implement kar liya tha

## Facilities provided by SIH

- They will reimburse sleeper class tickets up to 2k per person
- Accommodation will be provided by them i.e. hostel for boys and girls, quality depends upon the college

## Tech stack

### Backend

- We are decoupling the backend and frontend i.e. we won't be using Tauri rather there will be a server binary written in rust for performance and security which will communicate with frontend using REST API
- @Priyanshu and @Manav will be developing it for now!
- We will need to showcase them backend for the internal hackathon with a UI mockup

### Some references for backend

- @Priyanshu add krde

### Frontend

- We will be using GTK4 + Libadwaita with python bindings as everyone is familiar with python and since we are not using tauri so our other option will be to build it in [Nextron(Next + Electron)](https://github.com/saltyshiomix/nextron) which is just bloat and immature!
- I've explored(I == @Shivam) gtk4 in python and it's feasible but the documentation sucks!
- We will develop it after the architecture of backend is ready and the UI mockup is finished.

### Some references for gtk4 + python

- [Introduction to GTK4 in python](https://github.com/Taiko2k/GTK4PythonTutorial/tree/main) - this is a good starting point to get familiar with widget and development process of gtk4 + libadwaita in general.
- [GTK4-Python official documentation](https://docs.gtk.org/gtk4/)
- [GTK4 Widget gallery](https://docs.gtk.org/gtk4/visual_index.html)
- [Adwaita widget gallery](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/widget-gallery.html)
- Get familiar with [Gnome Builder](https://wiki.gnome.org/Apps/Builder)
  - [Documentation](https://developer.gnome.org/documentation/introduction/builder.html) - there are no tutorial available but the documentation is really nice.
- We will be using [Cambalache](https://gitlab.gnome.org/jpu/cambalache) or [Glade](https://github.com/GNOME/glade) for UI designing, i prefer cambalache!
  - [Glade documentation](https://help.gnome.org/users/glade/stable/)
  - [blog explining Cambalache](https://blogs.gnome.org/xjuan/)
- [Python + libadwaita demo app](https://github.com/timlau/adw_pydemo)

## Presentation

### SPOC Presentation

- We will have to strictly use this [template](https://www.sih.gov.in/letters/Idea-Presentation-Format-SIH2023-College.pptx).
- Two slides are just for problem statement and team information
- rest 2 slides will be used, try to include as much data as possible
- Take reference from [Venkatesh's ppt](https://cdn.discordapp.com/attachments/1150112553775677491/1150112604354781295/sih22.pdf)
- Try to include architecture design using draw.io.
- Include all the tech stack used. For eg, if we are using nextjs then mention nextjs, reactjs, js (this is just an example, we are not using Nextjs)

### Internal Presentation

- there is no limit on the number of slides so we can go berserk!
- @Shambhavi will prepare this!

## Work load distribution

### Till internal hackathon

- Backend: @Priyanshu, @Manav
- UI Mockup: @Shivam
- Presentation: @Shambhavi

### Afterwards

- Backend: @Priyanshu, @Manav, @Shivam
- UI Mockup: @Shivam, @Manav, @Shambhavi
- Frontend: @Shambhavi, @Tanishq, @Jigyasa, @Shivam
  
> @Everyone try to get familiar with the respective technologies in the mean time(#Cringe)

## References

### Curated

- [Best hardening guide](https://madaidans-insecurities.github.io/guides/linux-hardening.html)
- [Ubuntu specific hardening](https://wiki.ubuntu.com/Security/)
- [Awesome security hardening github](https://github.com/decalage2/awesome-security-hardening#ubuntu)
- Kernel hardening
  - <https://www.timesys.com/security/securing-your-linux-configuration-kernel-hardening/>
  - <https://www.kicksecure.com/wiki/Hardened-kernel>
- [Attack patterns - mitre](https://d3fend.mitre.org/)
- [CIS Benchmarking](https://downloads.cisecurity.org/#/)
- [CIS Benchmarking Ubuntu 22.04 LTS](https://learn.cisecurity.org/l/799323/2022-09-15/3l9d2k)

### Dump

- <https://www.cisecurity.org/benchmark/red_hat_linux>
- <https://github.com/decalage2/awesome-security-hardening>
- <https://www.nsa.gov/portals/75/documents/what-we-do/research/selinux/documentation/presentations/2005-flexible-support-for-security-policies-into-linux-os-presentation.pdf>
- <https://github.com/shaurya-007/NSA-Linux-Hardening-docs>
- <https://aws.amazon.com/blogs/opensource/security-features-of-bottlerocket-an-open-source-linux-based-operating-system/>
- <https://aws.amazon.com/blogs/containers/validating-amazon-eks-optimized-bottlerocket-ami-against-the-cis-benchmark/>
- <https://attack.mitre.org/>
- <https://d3fend.mitre.org/>
- <https://www.timesys.com/security/securing-your-linux-configuration-kernel-hardening/>
- <https://madaidans-insecurities.github.io/guides/linux-hardening.html>
- <https://www.kicksecure.com/wiki/Hardened-kernel>
