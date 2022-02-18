import type {GetServerSideProps} from 'next'
import Head from 'next/head'
import Image from 'next/image'
import styles from '../styles/Home.module.css'
import React, {useState} from "react";

const Home: React.FC<{ data: any }> = (props: { data: any }) => {

    return (
        <div className={styles.container}>
            <Head>
                <title>AutomoBible</title>
                <meta name="description" content="Find everything to repair your car"/>
                <link rel="icon" href="/favicon.ico"/>
            </Head>
            <img src={"/automobible-removebg.png"} style={{"width": '20%', justifyContent: 'center'}}/>
            <main className={styles.main}>

                <h1 className={styles.title}>
                    Welcome to <a href='/'>AutomoBible</a>
                </h1>

                <div className={styles.grid}>
                    {
                        props.data?.map((brand:{creation_year: string,name: string,image_url: string}) => (
                                <div className={styles.card}>
                                    <a key={`${brand.name}-${brand.creation_year}`} href={`/brand/${brand.name}`}>
                                        <h2>{brand?.name}</h2>
                                        <div className={styles.imagebox}>
                                            <img src={`${brand?.image_url}`}/>
                                        </div>
                                    </a>
                                </div>)
                        )
                    }
                </div>
            </main>

            <footer className={styles.footer}>
              footer
            </footer>
        </div>
    )
}

export const getServerSideProps: GetServerSideProps = async (context) => {
    // Fetch data from external API
    const res = await fetch(`http://localhost:1500/brands`)
    const data = await res.json()

    // Pass data to the page via props
    return {props: {data}}
}

export default Home
