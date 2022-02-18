import type {GetServerSideProps} from 'next'
import Head from 'next/head'
import styles from '../../styles/Home.module.css'
import React, {useState} from "react";
import {useRouter} from "next/router";

const Brand: React.FC<{ data: any }> = (props: { data: any }) => {

    const router = useRouter();
    const {name} = router.query;

    return (
        <div className={styles.container}>
            <Head>
                <title>{name}</title>
                <meta name="description" content={`Find everything to repair your ${name}`}/>
                <link rel="icon" href="/favicon.ico"/>
            </Head>
            <img src={"/automobible-removebg.png"} style={{"width": '20%', justifyContent: 'center'}}/>
            <main className={styles.main}>

                <h1 className={styles.title}>
                    <a href={`/brand/${name}`}>{name}</a>
                </h1>

                <div className={styles.grid}>
                    {
                        props.data?.map((brand:{creation_year: string,name: string,image_url: string}) => (
                            <div className={styles.card}>
                                <a key={`${brand.name}-${brand.creation_year}`} href="https://nextjs.org/docs">
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


    const params = context.params;
    console.log("params",params);
    const res = await fetch(`http://localhost:1500/models/${params?.name}`)
    const data = await res.json()

    // Pass data to the page via props
    return {props: {data}}
}

export default Brand
