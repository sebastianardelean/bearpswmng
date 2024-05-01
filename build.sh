#!/bin/bash
echo "Build script created by Sebastian M. Ardelean <sebastian.ardelean@cs.upt.ro>"
echo -e "Start deployment[$(date +"%T")]...\n"

#GLOBAL_VARIABLES
SW_NAME="bearpswmng"           #archive name
BUILD_DATE=$(date +%d.%m.%Y)    #build date
ARCH="amd64"
WORKING_DIRECTORY=$(pwd)        #save working directory
MAJOR_VERSION=0
MINOR_VERSION=0
BUILD_NUMBER=0
ERROR_NUMBER=0;


#FUNCTIONS
function compile_project() {            #function will accept 1 parameter and will be called compile_project "parameter"
    echo -e "\nCompiling $1...[$(date +"%T")]\n"
    make                                #first clean and then make the project
    ERROR_NUMBER=$(echo $?)             #get error number
    if [ $ERROR_NUMBER -ne 0 ]; then
        exit $ERROR_CODE
    fi

    }

function clean_project() {              #function will accept 1 parameter and will be called compile_project "parameter"
    echo -e "\nCleaning $1...[$(date +"%T")]\n"
    make clean
    ERROR_NUMBER=$(echo $?)             #get error number
    if [ $ERROR_NUMBER -ne 0 ]; then
        exit $ERROR_CODE
    fi

}



function extract_version() {                                    #function has no parameter
    versionFile=$(ls ./ | grep version.*)            #list bearpswmng directory and send result to grep to extract the file version.hpp
    
    #grep previous values from version.hpp
    MAJOR_VERSION=$(grep -Eo 'MAJOR_VERSION\s([0-9]{1,3})' ./$versionFile | grep -Eo '[0-9]{1,3}')
    ERROR_NUMBER=$(echo $?)             #get error number
    if [ $ERROR_NUMBER -ne 0 ]; then
        exit $ERROR_CODE
    fi
    
    MINOR_VERSION=$(grep -Eo 'MINOR_VERSION\s([0-9]{1,3})' ./$versionFile | grep -Eo '[0-9]{1,3}')
    ERROR_NUMBER=$(echo $?)             #get error number
    if [ $ERROR_NUMBER -ne 0 ]; then
        exit $ERROR_CODE
    fi
    BUILD_NUMBER=$(grep -Eo 'BUILD_NUMBER\s([0-9]{1,3})' ./$versionFile | grep -Eo '[0-9]{1,3}')
    ERROR_NUMBER=$(echo $?)             #get error number
    if [ $ERROR_NUMBER -ne 0 ]; then
        exit $ERROR_CODE
    fi
    
    #increment version logic
    BUILD_NUMBER=`expr $BUILD_NUMBER + 1`
    if [ $BUILD_NUMBER -ge 9 ]; then
        MINOR_VERSION=`expr $MINOR_VERSION + 1`
        BUILD_NUMBER=0
        
    fi
    
    if [ $MINOR_VERSION -ge 9 ]; then
        MINOR_VERSION=0
        MAJOR_VERSION=`expr $MAJOR_VERSION + 1`
    fi
    

    
}

function set_new_version_number() {                                             #function has no parameters
    versionFile=$(ls ./ | grep version.*)                                    #list bearpswmng directory and send result to grep to extract the file version.hpp
    sed -i "s/.*MAJOR_VERSION.*/#define MAJOR_VERSION ${MAJOR_VERSION}/" ./$versionFile
    sed -i "s/.*MINOR_VERSION.*/#define MINOR_VERSION ${MINOR_VERSION}/" ./$versionFile
    sed -i "s/.*BUILD_NUMBER.*/#define BUILD_NUMBER ${BUILD_NUMBER}/" ./$versionFile
    sed -i "s/.*BUILD_DATE.*/#define BUILD_DATE \"${BUILD_DATE}\"/" ./$versionFile
    }




function create_archive() {
    ###############################
    # Start Building Executable   #
    # with memory allocation      #
    ###############################

    #create buffer directory for saving .so files
    TYPE=$1
    FOLDER_NAME=$SW_NAME-$MAJOR_VERSION.$MINOR_VERSION.$BUILD_NUMBER-$BUILD_DATE\_$ARCH-$TYPE
    echo -e "\nCreating directory $FOLDER_NAME![$(date +"%T")]\n"
    mkdir $WORKING_DIRECTORY/$SW_NAME-$MAJOR_VERSION.$MINOR_VERSION.$BUILD_NUMBER-$BUILD_DATE\_$ARCH-$TYPE


     
    # Clean project
    clean_project

    #Compile Executable

    compile_project
    cp ./bin/bearpswmng $WORKING_DIRECTORY/$SW_NAME-$MAJOR_VERSION.$MINOR_VERSION.$BUILD_NUMBER-$BUILD_DATE\_$ARCH-$TYPE



    ###############################
    # Start Creating Archive      #
    ###############################
    echo -e "Creating deploy archive...[$(date +"%T")]\n"
    tar -zcvf $SW_NAME-$MAJOR_VERSION.$MINOR_VERSION.$BUILD_NUMBER-$BUILD_DATE\_$ARCH-$TYPE.tar.gz $SW_NAME-$MAJOR_VERSION.$MINOR_VERSION.$BUILD_NUMBER-$BUILD_DATE\_$ARCH-$TYPE


    ###############################
    # Start Creating Archive      #
    ###############################




    echo -e "Create usr/local/bin for executable...[$(date +"%T")]\n"
    mkdir -p $WORKING_DIRECTORY/$SW_NAME-$MAJOR_VERSION.$MINOR_VERSION.$BUILD_NUMBER-$BUILD_DATE\_$ARCH-$TYPE/usr/local/bin

    echo -e "Copy executable to usr/local/bin...[$(date +"%T")]\n"
    mv $WORKING_DIRECTORY/$SW_NAME-$MAJOR_VERSION.$MINOR_VERSION.$BUILD_NUMBER-$BUILD_DATE\_$ARCH-$TYPE/bearpswmng $WORKING_DIRECTORY/$SW_NAME-$MAJOR_VERSION.$MINOR_VERSION.$BUILD_NUMBER-$BUILD_DATE\_$ARCH-$TYPE/usr/local/bin






    ###############################
    # Create control file         #
    ###############################
    echo -e "\nCreating file...$(date +"%T")]\n"
    mkdir -p $WORKING_DIRECTORY/$SW_NAME-$MAJOR_VERSION.$MINOR_VERSION.$BUILD_NUMBER-$BUILD_DATE\_$ARCH-$TYPE/DEBIAN
    cd $WORKING_DIRECTORY/$SW_NAME-$MAJOR_VERSION.$MINOR_VERSION.$BUILD_NUMBER-$BUILD_DATE\_$ARCH-$TYPE/DEBIAN
    touch control



    echo "Package: bearpswmng" >> control
    echo "Version:" $MAJOR_VERSION.$MINOR_VERSION.$BUILD_NUMBER >> control
    echo "Architecture:" $ARCH >> control
    echo "Maintainer: Sebastian M. Ardelean <sebastian.ardelean@cs.upt.ro>" >> control
    echo "Description: Application to store passwords" >> control
    echo " bearpswmng is a command line application storing passwords." >> control
    echo -e "\n" >> control

    cd $WORKING_DIRECTORY

    dpkg-deb --build --root-owner-group $SW_NAME-$MAJOR_VERSION.$MINOR_VERSION.$BUILD_NUMBER-$BUILD_DATE\_$ARCH-$TYPE

    echo -e "Cleaning...[$(date +"%T")]\n"
    # delete directory used for creating archive.tar.gz
    rm -r $SW_NAME-$MAJOR_VERSION.$MINOR_VERSION.$BUILD_NUMBER-$BUILD_DATE\_$ARCH-$TYPE
    # clean obj and so files from projects
    clean_project
    clean_project
}


###############################
# PREPARING VERSION NUMBER    #
###############################
echo -e "\nExtracting version number![$(date +"%T")]\n"
extract_version


echo -e "\nSetting version number![$(date +"%T")]\n"
set_new_version_number


create_archive "release" ""

echo -e "Finished deployment![$(date +"%T")]\n"


