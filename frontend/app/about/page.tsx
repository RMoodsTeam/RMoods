import React from 'react'
import Section from './Section'


/**
 * About page, contains information about the project and the team.
 * @returns Element
 */
const about = () => {
    return (
        <>
            <div className='pt-5'>
                <h1 className='text-4xl font-bold text-center'>About</h1>
                <Section
                    title="Who Are We?"
                    content="We are a group of dedicated students passionate about technology and innovation. Our team is composed of individuals with
                    diverse backgrounds and expertise, all united by a common goal: to leverage the power of Natural Language Processing (NLP)
                    to analyze and derive insights from Reddit data. Our project aims to understand the dynamics of online communities and the
                    interactions within them. By applying advanced NLP techniques, we strive to uncover patterns and trends that can provide
                    valuable insights into user behavior and community engagement. We believe that our work can contribute to a deeper
                    understanding of social media and its impact on society."
                />
                <Section
                    title='Our Story'
                    content="Our journey began when we realized the immense potential of NLP in understanding social media dynamics.
                    Inspired by the growing influence of online communities, we decided to embark on a project that could provide
                    meaningful insights into user behavior and community interactions. This inspiration drove us to form a team
                    and start our project, aiming to make a difference in the field of data analysis."
                />
                <Section
                    title='Meet The Team'
                    content="Our team consists of Katarzyna Jabłońska, Sebastian Nowak, Michał Miłek, and Sebastian Jałoszyński.
                    Each of us brings unique skills and perspectives to the project, making our collaboration both dynamic and effective.
                    Together, we are committed to pushing the boundaries of what is possible with NLP and data analysis."
                />
                <Section
                    title='Our Evolution'
                    content="Throughout our project, we have faced numerous challenges, from technical obstacles to data interpretation issues.
                    However, each challenge has been an opportunity for growth and improvement. Our project has evolved significantly,
                    with continuous enhancements to our NLP models and data analysis techniques. Today, we are proud of the progress we have
                    made and the insights we have uncovered."
                />
            </div>
        </>
    )
}

export default about