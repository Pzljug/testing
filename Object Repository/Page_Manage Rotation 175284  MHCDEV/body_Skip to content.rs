<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Skip to content</name>
   <tag></tag>
   <elementGuidId>5d37a3ac-4162-4a74-a024-797966909413</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>fixed-header-body </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-gr-c-s-loaded</name>
      <type>Main</type>
      <value>true</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>cz-shortcut-listen</name>
      <type>Main</type>
      <value>true</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        Skip to content
                    

                                    
        
            
                
                    Toggle navigation
                    
                    
                    
                
				
            
            
				CareersWhat's New?TrainingHelp DeskContact Us
                        
							4 
                            327 pending invoices3 pending non-member entity entity affiliations1 administrative review items35 unread messages
                        
						
                        
							Ajay ArumugamAccount Menu 
                            
                                
                                    
										
    
        Bookmarks
    
    
                    
            
            
                
                    
                
                
                    
                        
                        Bookmark
                    
                
            
        
        
        Almost every page in ACEMAPP can be bookmarked here.  The input's title will default to the title of the page you are
        currently on, but it can be changed to your preference.  Clicking the bookmark button will add the bookmark
        to your list.
    



    $(function () {
        $(&quot;#bookmark_submit&quot;).click(function(e){
            e.preventDefault();
            var button = this;
            $.ajax({
                url: '/internal-bookmark',
                data: {
                    'action': 'add',
                    'bookmark_uri': window.location.pathname,
                    'bookmark_name': $(&quot;#bookmark_name&quot;).val()
                },
                dataType: 'json',
                method: 'POST',
                beforeSend: function () {
                    HoldOn.open(HoldOnOptions);
                },
                success: function (response) {
                    $(&quot;#bookmark_table&quot;).append('&lt;a href=&quot;'+window.location.pathname+'&quot; class=&quot;btn btn-info btn-sm btn-left btn-block&quot;>'+$(&quot;#bookmark_name&quot;).val()+'&lt;/a>');
                    $(button).closest('.panel-footer').slideUp();
                    HoldOn.close();
                    Messenger().success(response.message);
                },
                error: function (jqXHR, textStatus, errorThrown) {
                    var rawData = jqXHR.responseText;
                    bootbox.alert('ERROR:&lt;br>'+rawData);
                    HoldOn.close();
                }
            });
        });
        $(&quot;.shortcut_delete&quot;).click(function(e){
            e.preventDefault();
            var shortcut_id = $(this).data('shortcut');
            var link = this;
            $.ajax({
                url: '/internal-bookmark',
                data: {'action':'delete','shortcut_id':shortcut_id},
                dataType: 'json',
                method: 'POST',
                beforeSend: function () {
                    HoldOn.open(HoldOnOptions);
                },
                success: function (response) {
                    HoldOn.close();
                    Messenger().success(response.message);
                    $(link).closest('div').slideUp();
                },
                error: function (jqXHR, textStatus, errorThrown) {
                    var rawData = jqXHR.responseText;
                    bootbox.alert('ERROR:&lt;br>'+rawData);
                    HoldOn.close();
                }
            });
        });
    });


    
        Super Search
    
    
                    
                Entity Search
                
                    
                    
                        Search
                    
                
            
            
                            
                Member Search
                
                    
                    
                        Search
                    
                
            
            
                            
                Rotation Search
                
                    
                    
                        Search
                    
                
            
            
                            
                Job Search
                
                    
                    
                        Search
                    
                
            
            
            
Navigation Super Admin Health System Admin School System Admin Clinical User Company Admin School User Clinical Faculty Company Staff Observer Recruiter Student Candidate Utility User General MemberPersonal
        Manage
        Profile
        
            
                Inbox 35
                
            
            
                Affiliations0 Pending
                
            Exit MasqueradeRemove Bookmark
                    
                        $(function(){
                            $(&quot;.mask_bookmark&quot;).click(function(e){
                                e.preventDefault();
                                $.ajax({
                                    &quot;url&quot;:&quot;/admin/masquerade-bookmark&quot;,
                                    &quot;data&quot;:{
                                        &quot;action&quot;:$(this).data(&quot;action&quot;),
                                        &quot;member_id&quot;:$(this).data(&quot;member_id&quot;)
                                    },
                                    &quot;dataType&quot;:&quot;json&quot;,
                                    &quot;method&quot;:&quot;post&quot;,
                                    &quot;success&quot;:function(response) {
                                        if(response.success) {
                                            Messenger().error(response.message);
                                            $(&quot;.mask_bookmark&quot;).remove();
                                        } else {
                                            Messenger().success(response.message);
                                            $(&quot;.mask_bookmark&quot;).remove();
                                        }
                                    },
                                    &quot;beforeSend&quot;: function(){
                                        HoldOn.open(HoldOnOptions);
                                    },
                                    &quot;complete&quot;: function(){
                                        HoldOn.close();
                                    }
                                });
                            });
                        });
                    
                    Support0CE TrackerCalendarePortfolioLogout
                                    
                                
                            
                        
						
            
		
                            

            
                                        
            
                                                                
        
            
                
                    
                                            
                            
                                
                                                                    Partnerships
                                                            
                        
                    
                                            
                            
                                 Members
                                
                            
                            
                                
                                    
                                        AffiliationRotationCourse  Student  Student  Student  Clinical Faculty  Clinical Faculty  Clinical Faculty  Classroom Faculty  Classroom Faculty  Classroom Faculty  Preceptor  Preceptor  Preceptor  Company Staff  Company Staff  Company Staff  Volunteer  Volunteer  Volunteer  High School Student  High School Student  High School Student
                                    
                                
                            
                        
                    
                                            
                            
                                 Directory
                                
                            
                            
                                
                                    
                                        
                                            
                                                
                                                    
                                                    Entities
                                                
                                            
                                            
                                                
                                                    
                                                    Members
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                                            
                            
                                 View Rotations
                            
                        
                        
                            
                                 Create Rotation
                            
                        
                    
                                                            
                    
                        
                             Reporting
                        
                    

                                            
                            
                                 Document Manager
                            
                        
                    
                                                                
                            
                                
                            
                        
                                            
                            
                                
                            
                        
                                            
                            
                                
                            
                        
                                                                
                            
                                 Vendors
                            
                        
                    
                                            
                            
                                 Courses
                            
                        
                    
                                            
                            
                                 Surveys
                            
                        
                    
                                            
                            
                                 Evaluations
                            
                        
                    
                                            
                            
                                 Events
                            
                        
                    
                                            
                            
                                 Case Logs
                            
                        
                                                                
                            
                                 Time Logs
                            
                        
                                                                
                            
                                 Relationship Management
                            
                        
                                    

                
                    
                        
                             More 
                        
                        
                            
                                
                                 Relationship Management
                            
                                 Time Logs
                            
                                 Case Logs
                            
                                 Events
                            
                                 Evaluations
                            
                                                                            
                                             Broadcast
                                        
                                                                                                                
                                             Entity Profile
                                        
                                        
                                             Member Announcements
                                        
                                                                                                                                                                                                                                    
                                             Billing
                                        
                                                                                                                
                                             Permissions
                                        
                                                                                                                                                        
                                                 Requirement Config
                                            
                                                                    
                            
                        
                    

                                                            
                                            
                            
                                
                                Search
                                
                            
                            
                                
                                    
                                        
                                                                                            
                                                    Member Search
                                                    
                                                        
                                                            
                                                            
                                                                Search
                                                            
                                                        
                                                    
                                                
                                                                                                                                        
                                                    Rotation Search
                                                    
                                                        
                                                            
                                                            
                                                                Search
                                                            
                                                        
                                                    
                                                
                                                                                    
                                    
                                
                            
                        
                                    
            
        
    
    
        var entityMenuResizeTimeout_210;
        function resizeMenu_210() {
            var $navWrapper = $(&quot;#entity_nav_210&quot;);
            var $moreMenu   = $(&quot;#entity_nav_more_210&quot;);
            var $rightNav   = $navWrapper.find('.navbar-right');
            var $leftNav    = $navWrapper.find('.leftNav');
            var maxWidth    = $navWrapper.width() - $rightNav.width() - 15;
            var curWidth    = 0;

            $leftNav.find('> li').each(function(){
                var $li = $(this);
                var thisWidth = 0;
                if(!$li.attr('id')) {
                    $li.uniqueId();
                }
                var id = $li.attr('id');

                if($li.data('rw') > 0) {
                    thisWidth = $li.data('rw');
                } else {
                    thisWidth = $li.width();
                    $li.data('rw',thisWidth);
                }
                if(curWidth + thisWidth > maxWidth) {
                    if(!$li.hasClass('dropdown')) {
                        if (document.getElementById(id + '-child')) {
                            $('#' + id + '-child').show();
                        } else {
                            var $newButton = $li.find('a').clone();
                            $newButton.unwrap();
                            $newButton.addClass('btn btn-default btn-block btn-left');
                            $newButton.attr('id', id + '-child');
                            $moreMenu.prepend($newButton);
                        }
                        $li.hide();
                    }
                } else {
                    curWidth += thisWidth;
                    if(!$li.is(':visible')) {
                        $li.show();
                        $('#' + id + '-child').hide();
                        console.log($(id + '-child'));
                    }
                }
            });
        }
        $(function(){
            var uri = location.pathname;

            $(&quot;#entity_nav_210&quot;).find('.leftNav').find('> li').each(function(){
                $(this).find('a').each(function () {
                    if($(this).attr('href') == uri) {
                        $(this).addClass('active').addClass('bg-primary');
                        if($(this).hasClass('btn')) {
                            console.log('IS A BUTTON!');
                            // We are in a sub-menu. Track up!
                            $(this).closest('.dropdown').find('.dropdown-toggle').addClass('active').addClass('bg-primary');
                        }
                    }
                });
            });


            resizeMenu_210();
            setTimeout(resizeMenu_210(), 250);
            setTimeout(resizeMenu_210(), 1000);
            setTimeout(resizeMenu_210(), 2500);
            $(window).resize(function() {
                clearTimeout(entityMenuResizeTimeout_210);
                entityMenuResizeTimeout_210 = setTimeout(resizeMenu_210,250);
            });
        });
    

                                                                
                                            
                            

    
        
            
                 Home
            
            
        
    

                
                            
                    Entities
                
                        
        
                    
                            
                    School Home
                
                        
        
                    
                            
                    ACE College
                
                        
        
                    
                            
                    Rotations
                
                        
        
                
        
            Manage Rotation: 175284
            
        
    


                        
                                        
    
        Add Intro JS
        Add KB Article
    


    $(function(){
        $('.super_intro_add').click(function(){
            var type = $(this).data('type');
            var route = 'rotations_entity_manage';
            var title = 'Manage Rotation: 175284 | MHCDEV';
            var args = 'school|210|382505';

            $.ajax({
                'url': '/admin/kb/help/add-ajax',
                'data': {
                    'type': type,
                    'route': route,
                    'title': title,
                    'args': args
                },
                'dataType':'JSON',
                'method':'POST',
                'success':function(response){
                    window.open(response.url);
                }
            })

        });
    });


                    
                
                        
            
            
                                
                    
        
            Rotation #175284
            Comments &amp; HistoryConflictsFiles 0Member AuditCourse SearchReq. ListSub Members*Cache*
        
    

    
        
            

    
        
            Edit
        
                                    
                    * = required
                
                            
                                                            

    
                    

                                                                            
                            
                                            
                            Program
                                                                                
                    
                        BSN


                    
                                    
                        

                                                                            
                            
                                            
                            Experience
                                                                                
                    
                        Cohort Rotation


                    
                                    
                        

    
    
                                                                                    
                            
                                            
                            Entities
                                                                                
                    
                         ACE College  ACE hospital  


                    
                                    
                        

    



                                                                        
                            
                                            
                            Allow Faculty Document Access
                                                                                        
                                    
                                
                                                    
                    
                        
        
            
                 Yes
            
            
                  No
            
        
    


                    
                                    
                        


    
                                                                            
                            
                                            
                            Unit
                                                                                            *
            
                                                                                
                    
                            
    
                    Unit
        
                                    2 South
                                                3 North
                                                3 South
                                                4 North
                                                6 West
                                                Any available
                                                Behavioral Health
                                                Birthing Center
                                                Emergency Department
                                                ICU
                                                Infusion Center
                                                West Wing
                        Unit
    
        $(function(){
            $(&quot;#site_unit_id&quot;).select2({theme: &quot;bootstrap&quot;, width:'100%'});
        });
    


                    
                                    
                        

        
            Create New Unit
        
    
                                                                        
                            
                                            
                            Course
                                                                                
                    
                                    
    
                    Course
        
                                    111: Ultimate Test Course!
                                                N 001: Fundamentals
                                                N 005: test
                                                N 008: 
                                                N 101: 
                                                N 111: 
                                                N 118: 
                                                N 120: 
                                                N 121: 
                                                N 125: 
                                                N 126: 
                                                N 2206: 
                                                N 2256: 
                                                N 250: 
                                                TEST-01: TEST-01: QA-01
                        Course
    
        $(function(){
            $(&quot;#id_school_course_id&quot;).select2({
                theme: &quot;bootstrap&quot;,
                width:'100%',
                ajax: {
                    url: &quot;/entity/school/210/rotations/manage/382505/school_course&quot;,
                    method: &quot;GET&quot;,
                    dataType: &quot;json&quot;,
                    delay: 250,
                    data: function (params) {
                        let subData = {
                            q: params.term,
                            page: params.page
                        };
                        $.each($(&quot;#id_school_course_id&quot;).data(),function(i, e) {
                            if(typeof e != 'object'
                                &amp;&amp; typeof e != 'function'
                                &amp;&amp; i != 'q'
                                &amp;&amp; i != 'page'
                            ) {
                                subData[i] = e;
                            }
                        });

                        return subData;
                    },
                    processResults: function (data, page) {
                        return {
                            results: data
                        };
                    },
                    cache: false
                },

                templateSelection: function(result, container) {
                    return result.text;
                },
                templateResult: function(result) {
                    return result.text;
                },
                escapeMarkup: function (markup) { return markup; },
                minimumInputLength: 0,
                multiple: false
            });
        });
    



                    
                                    
                        

                                                                        
                        


    
Scheduling
                                                                    
                            
                                            
                            Student Slots
                                                                                            *
            
                                                                                
                    
                                
    

    

                    
                                    
                        


                                                                        
                            
                                            
                            Advanced Scheduling? Once enabled, cannot be disabled
                                                                                
                    
                        
        
            
                 Yes
            
            
                  No
            
        
    


                    
                                    
                        


                                                                        
                            
                                            
                            Start Date
                                                                                            *
            
                                                                                
                    
                                
    
    
    

        $('#id_rotation_date_start').datepicker({
            todayBtn: &quot;linked&quot;,
            todayHighlight: true
        }).on('changeDate', function(e){
            $(this).trigger('input');
        });
    



                    
                                    
                        

                                                                        
                            
                                            
                            End Date
                                                                                            *
            
                                                                                
                    
                                
    
    
    

        $('#id_rotation_date_end').datepicker({
            todayBtn: &quot;linked&quot;,
            todayHighlight: true
        }).on('changeDate', function(e){
            $(this).trigger('input');
        });
    



                    
                                    
                        



                                                                        
                            
                                            
                            Shift Start (ex 09:00)
                                                                                            *
            
                                                                                
                    
                        
    

                    
                                    
                        

                                                                        
                            
                                            
                            Shift End (ex 16:00)
                                                                                            *
            
                                                                                
                    
                        
    

                    
                                    
                        


                                                                    
                            
                                            
                            Days of the Week
                                                                                            *
            
                                                                                
                    
                                
    
        
                            Days of the Week
                                                            Monday
                                                                Tuesday
                                                                Wednesday
                                                                Thursday
                                                                Friday
                                                                Saturday
                                                                Sunday
                                    
        
            
                
            
        
        
            
                
            
        
    
    
        $(function(){
            $(&quot;#blob_dow&quot;).select2({theme: &quot;bootstrap&quot;, width:'100%'});
            $(&quot;#blob_dow-all&quot;).click(function(){
                var $sel = $(&quot;#blob_dow&quot;);
                var $sib = $sel.siblings('.select2');
                if($sib.hasClass('select2-container--disabled')){
                    return false;
                }
                $sel.find('option').each(function(){
                    if($(this).prop('disabled')) {
                        $(this).prop('selected', false);
                    } else {
                        $(this).prop('selected',true);
                    }
                });
                $sel.trigger('change');
            });
            $(&quot;#blob_dow-none&quot;).click(function(){
                var $sel = $(&quot;#blob_dow&quot;);
                var $sib = $sel.siblings('.select2');
                if($sib.hasClass('select2-container--disabled')){
                    return false;
                }
                $sel.find('option').each(function(){
                    $(this).prop('selected',false);
                });
                $sel.trigger('change');
            });
        });
    


                    
                                    
                        


                                                                                            
                            
                                            
                            Student Level
                                                                                
                    
                            
    
                    Student Level
        
                                    1st yr
                                                2nd yr
                                                3rd yr/jr
                                                4th yr/sr
                                                Graduate
                                                RNBSN
                                                2nd degree/accel
                                                Other
                                                Re-Entry
                        Student Level
    
        $(function(){
            $(&quot;#id_rotation_code_student_level&quot;).select2({theme: &quot;bootstrap&quot;, width:'100%'});
        });
    


                    
                                    
                        

                                                                                                
                            
                                            
                            Student Experience
                                                                                
                    
                            
    
                    Student Experience
        
                                    Beginner
                                                Intermediate
                                                Advanced
                        Student Experience
    
        $(function(){
            $(&quot;#id_rotation_code_student_experience&quot;).select2({theme: &quot;bootstrap&quot;, width:'100%'});
        });
    


                    
                                    
                        

                                                                                            
                            
                                            
                            Nursing Type
                                                                                            *
            
                                                                                
                    
                            
    
                    Nursing Type
        
                                    Not Set
                                                Ambulatory
                                                Cardiovascular
                                                Community
                                                Critical/Intensive Care
                                                Emergency
                                                Internal Medicine
                                                Leadership
                                                MedSurg - Acute Care
                                                Neonatal ICU
                                                Observation
                                                Other (see comments)
                                                Outpatient
                                                Pediatrics
                                                Perinatal - OB
                                                Post-Acute
                                                Practicum
                                                Psychiatry; Mental Health; Behavioral Medicine
                                                Research
                                                Stepdown/Progressive
                                                Surgical / OR
                                                To Be Determined
                        Not Set
    
        $(function(){
            $(&quot;#id_type_id&quot;).select2({theme: &quot;bootstrap&quot;, width:'100%'});
        });
    


                    
                                    
                        

                            

                                                                                        
                            
                                            
                            Faculty Name Entering a name does not assign a faculty
                                                                                
                    
                        
    

                    
                                    
                        

                

                                                                    
                            
                                            
                            Status
                                                                                            *
            
                                                                                
                    
                            
    
                    Status
        
                                    Bulk
                                                Pending
                                                Needs Info
                                                Approved
                                                Completed
                                                Denied
                                                Withdrawn
                                                Archived
                                                Deleted
                                                Purged
                        Status
    
        $(function(){
            $(&quot;#id_status&quot;).select2({theme: &quot;bootstrap&quot;, width:'100%'});
        });
    


                    
                                    
                        


                                                                        
                            
                                            
                            Withdraw Reason
                                                                                
                    
                        


                    
                                    
                        



                                                                        
                            
                                            
                            Deny Reason
                                                                                
                    
                        


                    
                                    
                        


    
    Custom Fields
                
                                                        
                                            
                                                                                                
                            
                                            
                            Is this a rotation?
                                                                                    
                                                                                
                    
                        
        
            
                 Yes
            
            
                 No
            
        
    


                    
                                    
                        

                        
                                    
                                                                            
    




    $(function(){
        var siteUnitSelect = $(&quot;#site_unit_id&quot;);
        siteUnitSelect.change(function(){hackConflictChecker();});
        // Select2 extras!
        siteUnitSelect.on('select2:select', function () {
            hackConflictChecker();
        });
        siteUnitSelect.on('select2:close', function () {
            hackConflictChecker();
        });

        siteUnitSelect.on('select2:unselect', function () {
            hackConflictChecker();
        });

        $('input[name=&quot;rotation_date_start&quot;]').change(function(){hackConflictChecker();});
        $('input[name=&quot;rotation_date_end&quot;]').change(function(){hackConflictChecker();});
        $('input[name=&quot;rotation_shift_start&quot;]').change(function(){hackConflictChecker();});
        $('input[name=&quot;rotation_shift_end&quot;]').change(function(){hackConflictChecker();});
        $('#blob_dow').change(function(){hackConflictChecker();});
        $('#id_status').change(function(){withdrawChecker();});

        // Run once
        withdrawChecker();

        if($(&quot;#link_site_unit&quot;).length > 0) {
            $(&quot;#site_unit_add_div&quot;).show();
            $(&quot;#site_unit_add_button&quot;).click(function(e){
                e.preventDefault();
                $(&quot;#link_site_unit&quot;).trigger('click');
            });
        }

        $(&quot;#id_rotation_ind_advanced_scheduling&quot;).find('input').change(function () {
            if($(&quot;input[name='rotation_ind_advanced_scheduling'][value='1']&quot;).prop(&quot;checked&quot;)) {
                bootbox.confirm({
                    title: &quot;Advanced Scheduling Warning&quot;,
                    message: &quot;Once advanced scheduling is enabled for a rotation, it cannot be turned off again. Updating the start date, end date, shift start, shift end or days of the week will reset changes made to the advanced schedule.&quot;,
                    buttons: {
                        confirm: {
                            label: &quot;Ok&quot;,
                            className: &quot;btn-success&quot;
                        },
                        cancel: {
                            label: &quot;Cancel&quot;,
                            className: &quot;btn-danger&quot;
                        }
                    },
                    callback: function(result) {
                        if(!result) {
                            $(&quot;input[name='rotation_ind_advanced_scheduling'][value='0']&quot;).trigger('click');
                        }
                    }
                });
            }
        });
    });
                    
        
                            
                    
    

    
        $(function(){
            var FC_HOTrip = false;
            $(&quot;#form_edit:not(.FC_BOUND)&quot;).on('submit.FC',function(){
                if(!FC_HOTrip) {
                    HoldOn.open(HoldOnOptions);
                    FC_HOTrip = true;
                }
            });
        });
    



        
        
            
    
        Comments
                
    
    
                    
                New Comment
                
                
            
            
    
        
                            No Comments Found
                    
    

    
        $('.deleteComment').click(function(){
            var commentId = $(this).data('commentid');
            var commentText = $(this).siblings('.commentBody').html();
            var comment = $(this).closest('.individualComment');
            var data = {'function' : 'remove_comment', 'comment_id' : commentId};
            bootbox.dialog({
                'title'   : 'Remove the following comment?',
                'message' : commentText,
                'buttons' : {
                    'confirm' : {
                        'label'     : 'Yes',
                        'className' : 'btn-success',
                        'callback'  : function(result){
                            $.ajax({
                                'url'    : '/member/school/rotations/manage/382505/comm_delete',
                                'method' : 'POST',
                                'data'   : data,
                                'success': function(subData){
                                    if(subData.success){
                                        comment.remove();
                                    } else {
                                        bootbox.alert(subData.message)
                                    }
                                }
                            });
                        }
                    },
                    'cancel'  : {
                        'label'     : 'No',
                        'className' : 'btn-warning'
                    }
                }
            });
        });
    
History
    Brian Clauser - 09/18/2019 1:31:01pm
    
        
            
                Field
                Before
                After
            
        
        
                            
                    Program
                    not set
                    BSN
                
                            
                    Experience
                    not set
                    Cohort Rotation
                
                    
    


        
    

    
        var myObj = new RotationManager('/entity/school/210/rotations/manage/382505/menu');

        $(function(){
            myObj.loadMenu();
        });

        function RotationManager(route) {
            this.rightLoaded = false;
            this.menuRoute = route;
            this.loadedItem = 'comm_hist';

            this.loadMenu = function () {
                var main = this;
                $.getJSON(this.menuRoute, function(data){
                    $.get(data.edit[1], function(subData){
                        var form = $('#rotation_form');
                        form.html(subData);
                        form.find('form').attr('action',data.edit[1]).attr('id','form_edit');
                    },'html');
                    if(main.rightLoaded == false) {
                        main.rightLoaded = true;
                        $.get(data.comm_hist[1], function(subData){
                            var extras = $('#rotation_extras');
                            extras.html(subData);
                            extras.find('form').attr('action',data.comm_hist[1]).attr('id','form_comm_hist');
                        },'html');
                    }

                    menu = $(&quot;#rotation_menu&quot;);
                    menu.html('');
                    for (var property in data) {
                        if (data.hasOwnProperty(property)) {
                            if(property != 'edit') {
                                var tmp = data[property][1].split('/');
                                var name = tmp[tmp.length-1];
                                if(data[property][2]) {
                                    if(name == main.loadedItem) {
                                        menu.append('&lt;li>&lt;a href=&quot;#&quot; class=&quot;active&quot; id=&quot;link_'+name+'&quot; data-id=&quot;link_'+name+'&quot; data-name=&quot;'+name+'&quot; data-route=&quot;' + data[property][1] + '&quot;>' + data[property][0] + '&lt;/a>&lt;/li>');
                                    } else {
                                        menu.append('&lt;li>&lt;a href=&quot;#&quot; id=&quot;link_'+name+'&quot; data-id=&quot;link_'+name+'&quot; data-name=&quot;'+name+'&quot; data-route=&quot;' + data[property][1] + '&quot;>' + data[property][0] + '&lt;/a>&lt;/li>');
                                    }
                                } else {
                                    menu.append('&lt;li>&lt;a style=&quot;display:none&quot; href=&quot;#&quot; id=&quot;link_'+name+'&quot; data-id=&quot;link_'+name+'&quot; data-name=&quot;'+name+'&quot; data-route=&quot;' + data[property][1] + '&quot;>' + data[property][0] + '&lt;/a>&lt;/li>');
                                }
                            }
                        }
                    }
                });
            }
        }

        $(document).on('click','#rotation_menu a', function(){
            var menu = $(&quot;#rotation_menu&quot;);
            menu.find('.active').removeClass('active');
            $(this).parent().addClass('active');
            myObj.loadedItem = $(this).data('name');

            var route = $(this).data('route');
            var tmp = route.split('/');
            var name = tmp[tmp.length-1];
            $.ajax({
                url: route,
                dataType:'html',
                method: 'GET',
                beforeSend: function () {
                    if($(&quot;#holdon-overlay&quot;).length &lt; 1) {
                        HoldOn.open(HoldOnOptions);
                    }
                },
                complete: function() {
                    HoldOn.close();
                },
                success: function(data) {
                    var extras = $(&quot;#rotation_extras&quot;);
                    extras.html(data);
                    extras.find('form').attr('action', route).attr('id','form_'+name);
                }
            });
        });

        $(document).on('submit','form',function(e){
            if($(this).hasClass('search-form')) {
                return true;
            }
            if($(this).attr('target')=='_blank') {
                return true;
            }

            e.preventDefault();
            var form     = $(this);
            var url      = form.attr('action');
            var postData = form.serialize();
            var name     = form.attr('id');

            $.ajax({
                url: url,
                data: postData,
                dataType: 'json',
                method: 'POST',
                beforeSend: function () {
                    HoldOn.open(HoldOnOptions);
                },
                success: function(data){
                    $(&quot;html, body&quot;).animate({ scrollTop: 0 }, &quot;slow&quot;);
                    if(data.success) {
                        if(data.hasOwnProperty('message')) {
                            Messenger().success(data.message);
                        } else {
                            Messenger().success('Form saved successfully!');
                        }
                        myObj.loadMenu();
                        if($(&quot;#rotation_extras&quot;).has(form).length) {
                            $(&quot;a[data-route='&quot;+url+&quot;']&quot;).trigger('click');
                        } else {
                            HoldOn.close();
                            $(&quot;#link_comm_hist&quot;).trigger('click');
                        }
                    } else {
                        if ('message' in data) {
                            bootboxError(data.message);
                            // Messenger().error(data.message);
                        } else {
                            bootboxError('There was an error with the form');
                            // Messenger().error('There was an error with the form');
                        }
                        $(&quot;.ajaxError&quot;).remove();
                        var errors = data.errors;
                        for (var property in errors) {
                            if (errors.hasOwnProperty(property)) {
                                var item = $('input[name=&quot;' + property + '&quot;], select[name=&quot;' + property + '&quot;], textarea[name=&quot;' + property + '&quot;]');
                                var block = item.closest('.form-group ');
                                block.append('&lt;div class=&quot;label label-danger ajaxError&quot; style=&quot;white-space:normal&quot;>' + errors[property] + '&lt;/div>');
                            }
                        }
                        HoldOn.close();
                    }
                },
                error: function(jqXHR, textStatus, errorThrown) {
                    var rawData = jqXHR.responseText;
                    bootbox.alert('ERROR:&lt;br>' + rawData);
                    HoldOn.close();
                }
            });

        });

        $(document).on('click', '.bootme', function() {
            HoldOn.open(HoldOnOptions);
            var url = $(this).data('booturl');

            $.ajax({
                'url': url,
                'dataType':'html',
                'success': function(data) {
                    HoldOn.close();
                    bootbox.dialog({
                        message: data,
                        className: 'bootbox-wide',
                        title: 'Member Info',
                        ok: {
                            label: &quot;Ok&quot;,
                            className: &quot;btn-default&quot;,
                            callback: function () {
                            }
                        }
                    });
                },
                error: function(jqXHR, textStatus, errorThrown) {
                    var rawData = jqXHR.responseText;
                    bootbox.alert('ERROR: &lt;br>' + rawData);
                    HoldOn.close();
                }
            });
        });

        function hackConflictChecker()
        {
            if($(&quot;#link_conflicts&quot;).length > 0) {

                var url = $(&quot;#link_conflicts&quot;).data('route') +
                        '?site_unit_id=' + $('#site_unit_id').val() +
                        '&amp;rotation_date_start=' + $('input[name=&quot;rotation_date_start&quot;]').val() +
                        '&amp;rotation_date_end=' + $('input[name=&quot;rotation_date_end&quot;]').val() +
                        '&amp;rotation_shift_start=' + $('input[name=&quot;rotation_shift_start&quot;]').val() +
                        '&amp;rotation_shift_end=' + $('input[name=&quot;rotation_shift_end&quot;]').val() +
                        '&amp;dow=' + $('#blob_dow').val();

                $.ajax({
                    url: url,
                    dataType: 'html',
                    method: 'GET',
                    beforeSend: function () {
                        if ($(&quot;#holdon-overlay&quot;).length &lt; 1) {
                            HoldOn.open(HoldOnOptions);
                        }
                    },
                    complete: function () {
                        HoldOn.close();
                    },
                    success: function (data) {
                        var extras = $(&quot;#rotation_extras&quot;);
                        extras.html(data);
                    }
                });
            }
        }

        function withdrawChecker()
        {
            if($(&quot;#id_status&quot;).val() == 'W') {
                $(&quot;#withdraw_reason&quot;).show();
            } else {
                $(&quot;#withdraw_reason&quot;).hide();
            }

            if($(&quot;#id_status&quot;).val() == 'Y') {
                $(&quot;#deny_reason&quot;).show();
            } else {
                $(&quot;#deny_reason&quot;).hide();
            }
        }

    
            
        
                    
                                
                    
                        © 2019 MHC | ACEMAPP
                    
                    
                                                    2.14.48
                                                
                                                
                            Build:  Success
                        
                    
                    
                        Terms &amp; Privacy
                    
                
            
            
                
                    
                        DEBUG INFO
                    
                    
                        Server Name: mhcdev.comRoute Name: rotations_entity_manageLoad: 0.22 0.11 0.04Duration: 0.249sI Am: Brian Clauser (176720)Mask: Ajay Arumugam (6)Time: 09/18/2019 1:31pm EDTRoute Arguments: school | 210 | 382505Route Info: pattern: /entity/:alpha/:number/rotations/manage/:number |    class: \MHC\ACE2\Handlers\Rotations\Manage |    autoWrapper: 1 |    session: 1 |    preArgs: Array
(
    [0] => entity
)
 |    title: Manage Rotation (Entity) |    meta: Array
(
)
 |    Twig Files: pseudo/rotations/manage_controller.twig | pseudo/pagewrapper/bookmarks.twig | pseudo/pagewrapper/super_search.twig | views/help_button_super.twigPEX Master ListPEX (STANDARD)  admin.misc.kb.help.edit > 2PEX (ANY)-NONE-PEX (MAX)  admin.billing > 2  admin.misc.custom_forms > 2  admin.report.pending_nonmember_entity_entity  > 2  admin.misc.admin_review > 2  admin.entity > 2  admin.member > 2  admin.rotation > 2  admin.opportunity > 2PEX (EXACT)-NONE-URI History/entity/school/210/rotations/manage/382505/entity/school/210/rotations/add/84101/entity/school/210/rotations/add/84101/entity/school/210/rotations/add/84101/entity/school/210/rotations/add?q=ace&amp;reqlevel=%221%22&amp;oldTabindex=0&amp;_=1568827631267/entity/school/210/rotations/add?q=ac+e&amp;reqlevel=%221%22&amp;oldTabindex=0&amp;_=1568827631266/entity/school/210/rotations/add/api/misc/data-table-state/entity/school/210/rotations/entity/school/210/rotations/counts?_=1568827627446
                    
                
                        
                sessionStorage.setItem('login_state','1');
                setTimeout(function(){
                    doKeepAlive();
                    setInterval(function(){
                        doKeepAlive();
                    },60000)
                },500);

                function doKeepAlive() {
                    $.ajax({
                        'url':'/keep-alive',
                        'dataType':'JSON',
                        'method':'GET',
                        'success':function(response){
                            var state = sessionStorage.getItem('login_state');
                            if(state > 0 &amp;&amp; !response.logged_in) {
                                window.location.href = window.location.protocol + '//' + window.location.hostname;
                            }
                        }
                    });
                }

                //var location = window.location;
                var path = window.location.pathname;
                $.each($('#site_header>.navbar>.collapse>.nav>li>a'), function(){
                    var $link = $(this);
                    var href  = $link.attr('href');

                    if (href == path) {
                        $link.closest('li').addClass('active');
                    }
                });
            
                            
            
                
            
        
    
/html[@class=&quot;b_chrome o_windows gr__mhcdev_com&quot;]/body[@class=&quot;fixed-header-body&quot;]
.tagove_frame{max-width:100%!important;display:block!important;z-index:2147483647!important;min-height:60px!important;top:auto;bottom:0!important;left:auto;right:0!important;min-width:116px!important;margin-bottom:0!important}.tagove_frame.hide_frame,.tagove_frame.status_noui{display:none!important}.tagove_frame.ui_status_max{min-width:400px!important}.tagove_frame.ui_status_max.status_chat_form{min-width:300px!important}.tagove_frame.ui_status_max.a-dfdde{min-width:200px!important}.tagove_frame.ui_status_min{min-width:116px!important}.tagove_frame.show_zero{display:none!important}.tagove_frame.a-dfdde{min-width:200px!important}.shadow-acquirebg{z-index:2147482998;position:fixed;width:500px;height:500px;bottom:0;right:0;content:&quot;&quot;;pointer-events:none;background:radial-gradient(ellipse at bottom right,rgba(29,39,54,.16) 0,rgba(29,39,54,0) 72%)}.status_chat.ui_status_min{min-width:100px!important;min-height:80px!important}.status_chat.ui_status_min.agent-online{max-width:100px!important}#tg_ui_frame{border:0}#tg_ui_frame.loading{background:red}.button_modern.status_agent_list{min-width:125px!important;width:125px!important}.button_modern.ui_status_min{min-width:116px!important;height:116px!important;bottom:0!important;right:0!important}.button_modern.ui_status_min.plain_l{min-width:60px!important;height:60px!important;bottom:20px!important;right:20px!important}.button_modern.ui_status_min.plain_l.pos_bl{right:auto!important;left:20px!important}.button_modern.ui_status_min.pos_bl{right:auto!important;left:0!important}.pos_bl{top:auto;bottom:0!important;left:0!important;right:auto;text-align:left;float:right;display:inline-block}.mobile-frame .tagove_frame{background-color:transparent;vertical-align:text-bottom;top:0;left:0;width:100%;height:100%;position:fixed;z-index:16000003;min-width:100%;max-width:100%;min-height:100%;max-height:100%;display:block;opacity:1;transform:translateY(0%);padding:0;margin:0}.mobile-frame .tagove_frame .status_offline-form-success,.mobile-frame .tagove_frame.status_agent_list,.mobile-frame .tagove_frame.status_chat_form,.mobile-frame .tagove_frame.status_offline-form,.mobile-frame.cobrowseonly-widget .tagove_frame.status_agent_list.agent-online{min-width:280px!important;width:280px!important}.mobile-frame .tagove_frame.widget-open{height:256px!important}.mobile-frame .tagove_frame.button_modern.status_agent_list{height:100px!important;min-width:105px!important;width:105px!important}.mobile-frame .tagove_frame.ui_status_max{min-width:100%!important}.mobile-frame .tagove_frame.ui_status_max.status_chat_form{min-width:50%!important}.mobile-frame .tagove_frame.status_agent_list.agent-online{min-width:105px!important;width:105px!important}.mobile-frame .mobile_focus.status_chat_form,.mobile-frame .mobile_focus.status_offline-form{vertical-align:text-bottom;top:0!important;left:0!important;bottom:0!important;right:0!important;position:fixed!important;width:100%!important;min-width:100%!important;min-height:100%!important}.cobrowseonly-widget .tagove_frame.a-98953,.cobrowseonly-widget .tagove_frame.a-f82eb{min-width:280px!important;padding-top:0!important}.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb{min-width:320px!important;width:320px!important}.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.widget-open,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.widget-open{min-height:340px!important;min-width:100%!important;width:100%!important}.cobrowseonly-widget .tagove_frame.a-98953.ui_status_max,.cobrowseonly-widget .tagove_frame.a-f82eb.ui_status_max{min-width:675px!important;min-height:343px!important;height:343px!important}.cobrowseonly-widget .tagove_frame.widget-open{min-width:675px!important;height:343px!important}.cobrowseonly-widget.mobile-frame .tagove_frame{min-width:280px!important;width:280px!important;min-height:55px!important}.cobrowseonly-widget.mobile-frame .tagove_frame.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.widget-open{min-height:220px!important;min-width:100%!important;width:100%!important}.cobrowseonly-widget .tagove_frame{min-width:320px!important}.cobrowseonly-widget .tagove_frame.ui_status_max,.cobrowseonly-widget .tagove_frame.widget-open{min-height:220px!important}.cobrowseonly-widget .tagove_frame.widget-connected.status_chat.ui_status_max{height:55px!important}.tagove-livechat-widget-popup{position:fixed;top:10%!important;left:3%;right:auto!important;width:70%;height:80vh;z-index:3000000000;background:#fff;border:2px solid transparent;box-shadow:0 0 7px 4px rgba(0,0,0,.19)}.tagove-livechat-widget-popup .popup-wrap{height:100%;width:100%;position:relative}#tg_ui_frame,.tagove-livechat-widget-popup .popup-wrap iframe{width:100%;height:100%}.tagove-livechat-widget-popup .popup-wrap .resize{position:absolute;right:-7px;bottom:-7px;cursor:se-resize}.tagove_frame.a-09a05.button_simple.ui_status_min,.tagove_frame.a-62019.button_simple.ui_status_min,.tagove_frame.a-84389.button_simple.ui_status_min{min-width:190px!important;width:190px!important}.mobile-frame .tagove_frame.a-09a05.button_simple.widget-open,.mobile-frame .tagove_frame.a-62019.button_simple.widget-open,.mobile-frame .tagove_frame.a-84389.button_simple.widget-open{height:256px!important}.mobile-frame .tagove_frame.a-09a05.button_simple.status_agent_list.agent-online,.mobile-frame .tagove_frame.a-62019.button_simple.status_agent_list.agent-online,.mobile-frame .tagove_frame.a-84389.button_simple.status_agent_list.agent-online{height:70px!important;min-width:75px!important;width:75px!important}.mobile-frame.ios-frame .tagove_frame.a-09a05.ui_status_min,.mobile-frame.ios-frame .tagove_frame.a-62019.ui_status_min,.mobile-frame.ios-frame .tagove_frame.a-84389.ui_status_min{min-width:75px!important;width:75px!important}.mobile-frame.cobrowseonly-widget .tagove_frame.a-09a05.button_simple.agent-online,.mobile-frame.cobrowseonly-widget .tagove_frame.a-62019.button_simple.agent-online,.mobile-frame.cobrowseonly-widget .tagove_frame.a-84389.button_simple.agent-online{min-width:280px!important;width:280px!important}.tagove_frame.a-1556d.status_agent_list,.tagove_frame.a-b5662.status_agent_list{min-height:45px!important}.tagove_frame.a-1556d.status_chat.ui_status_min,.tagove_frame.a-b5662.status_chat.ui_status_min{min-width:300px!important;width:300px!important;max-height:90px}.mobile-frame .tagove_frame.a-1556d.status_agent_list.agent-online,.mobile-frame .tagove_frame.a-1556d.status_chat.ui_status_min,.mobile-frame .tagove_frame.a-b5662.status_agent_list.agent-online,.mobile-frame .tagove_frame.a-b5662.status_chat.ui_status_min{min-width:90px!important;width:90px!important}.tagove_frame.a-1556d.ui_status_min,.tagove_frame.a-b5662.ui_status_min{min-height:46px!important}.mobile-frame .tagove_frame.button_custom.status_agent_list.agent-online.a-91d17,.mobile-frame .tagove_frame.button_custom.status_agent_list.agent-online.a-a82c2,.mobile-frame .tagove_frame.button_custom.status_agent_list.agent-online.a-be477{min-width:300px!important;width:300px!important}@media (max-width:400px){.tagove_frame{min-width:100%!important;padding:0}}@media screen and (max-width:650px){.tg-url{top:-4px!important;left:0!important;width:100%!important;height:70px!important}.tg-url.hide{top:-72px!important}.tg-url iframe{height:68px!important}.tg-url a{top:67px!important}}@media (max-device-width:1536px){.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.widget-open,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.widget-open{min-height:282px!important}}@media (max-device-width:1192px){.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.widget-open,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.widget-open{min-height:300px!important}}@media (max-device-width:909px){.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.widget-open,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.widget-open{min-height:318px!important}}@media (max-device-width:854px){.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.widget-open,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.widget-open{min-height:313px!important}}@media (max-device-width:804px){.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.widget-open,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.widget-open{min-height:332px!important}}@media (max-device-width:750px){.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.widget-open,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.widget-open{min-height:338px!important}}@media (max-device-width:710px){.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.widget-open,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.widget-open{min-height:330px!important}}@media (max-device-width:700px){.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-98953.widget-open,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.ui_status_max,.cobrowseonly-widget.mobile-frame .tagove_frame.a-f82eb.widget-open{min-height:500px!important}}@media (max-width:500px){.tagove-livechat-widget-popup{width:98%;left:1%;top:15%!important;height:70vh}}@media (max-width:770px){.mobile-frame .tagove_frame.a-b5662.button_simple.status_agent_list{bottom:10px!important;right:70px!important}}</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;b_chrome o_windows gr__mhcdev_com&quot;]/body[@class=&quot;fixed-header-body&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
   </webElementXpaths>
</WebElementEntity>
