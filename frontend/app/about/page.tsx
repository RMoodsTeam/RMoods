import React from 'react'
import WhoAreWe from './WhoAreWe'
import OurStory from './OurStory'
import MeetTheTeam from './MeetTheTeam'
import OurEvolution from './OurEvolution'

const about = () => {
    return (
        <>
            <div className='pt-5'>
                <h1 className='text-4xl font-bold text-center'>About</h1>
                <WhoAreWe />
                <OurStory />
                <MeetTheTeam />
                <OurEvolution />
            </div>
        </>
    )
}

export default about